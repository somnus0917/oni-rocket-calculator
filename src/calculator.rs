use crate::models::*;
pub struct CalculatorInput {
    pub rocket: RocketInput,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LimitingResource {
    Fuel,
    Oxidizer,
    Balance,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CalculatorResult {
    pub restrict: LimitingResource,
    pub exact_range: f32,
    pub complete_range: u32,
    pub speed: f32,
    pub total_height: u32,
    pub total_burden: u32,
}
#[derive(Debug, PartialEq, Clone)]
pub enum CalculatorError {
    NegativeFuel,
    MissingOxidizer,
    NegativeOxidizer,
    FuelExceedsCapacity,
    OxidizerExceedsCapacity,
    IncompatibleOxidizerTank,
    RocketExceedsMaxHeight,
    CommandModuleTooMuch,
    CommandModuleTooLess,
    ConeModuleTooMuch,
    ConeModuleTooLess,
}

pub fn calculate(input: CalculatorInput) -> Result<CalculatorResult, CalculatorError> {
    let rocket = input.rocket;
    let engine = rocket.engine.spec();
    let fuel = rocket.fuel_amount;
    if fuel < 0.0 {
        return Err(CalculatorError::NegativeFuel);
    }
    let module_height: u32 = rocket.modules.iter().map(|module| module.height()).sum();
    let command_module_count = rocket
        .modules
        .iter()
        .filter(|module| matches!(module, RocketModule::Spacefarer(_)))
        .count();
    if command_module_count == 0 {
        return Err(CalculatorError::CommandModuleTooLess);
    }
    if command_module_count > 1 {
        return Err(CalculatorError::CommandModuleTooMuch);
    }
    let nosecone_count = rocket
        .modules
        .iter()
        .filter(|module| {
            matches!(
                module,
                RocketModule::Nosecone(_)
                    | RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone)
            )
        })
        .count();
    if nosecone_count == 0 {
        return Err(CalculatorError::ConeModuleTooLess);
    }
    if nosecone_count > 1 {
        return Err(CalculatorError::ConeModuleTooMuch);
    }
    let total_height = engine.height + module_height;
    if total_height > engine.max_rocket_height {
        return Err(CalculatorError::RocketExceedsMaxHeight);
    }

    let module_burden: u32 = rocket.modules.iter().map(|module| module.burden()).sum();
    let total_burden = engine.burden + module_burden;
    let speed = engine.engine_power as f32 / total_burden as f32;
    match engine.fuel_storage {
        FuelStorage::Internal { capacity } => {
            if fuel > capacity {
                return Err(CalculatorError::FuelExceedsCapacity);
            }
        }
        FuelStorage::ExternalTank => {
            let total_capacity: f32 = rocket
                .modules
                .iter()
                .filter_map(|module| match module {
                    RocketModule::FuelTank(tank) => Some(*tank),
                    _ => None,
                })
                .map(|tank| tank.spec().capacity)
                .sum();
            if total_capacity < fuel {
                return Err(CalculatorError::FuelExceedsCapacity);
            }
        }
    }
    let fuel_per_hex = engine.fuel_per_hex;
    let (exact_range, restrict) = if engine.requires_oxidizer {
        let oxi = rocket
            .oxidizer_input
            .ok_or(CalculatorError::MissingOxidizer)?;
        if oxi.oxidizer_amount < 0.0 {
            return Err(CalculatorError::NegativeOxidizer);
        }
        let compatible = rocket
            .modules
            .iter()
            .filter_map(|module| match module {
                RocketModule::OxidizerTank(tank) => Some(*tank),
                _ => None,
            })
            .all(|tank| tank.spec().storage_kind == oxi.oxidizer.storage_kind());
        if !compatible {
            return Err(CalculatorError::IncompatibleOxidizerTank);
        }
        let total_oxidizer_capacity: f32 = rocket
            .modules
            .iter()
            .filter_map(|module| match module {
                RocketModule::OxidizerTank(tank) => Some(*tank),
                _ => None,
            })
            .map(|tank| tank.spec().capacity)
            .sum();
        if oxi.oxidizer_amount > total_oxidizer_capacity {
            return Err(CalculatorError::OxidizerExceedsCapacity);
        }
        let effective_oxi = oxi.oxidizer_amount * oxi.oxidizer.spec().efficiency;
        let range = fuel.min(effective_oxi) / fuel_per_hex;
        let res = match fuel - effective_oxi {
            x if x > 1e-6 => LimitingResource::Oxidizer,
            x if x.abs() < 1e-6 => LimitingResource::Balance,
            _ => LimitingResource::Fuel,
        };
        (range, res)
    } else {
        (fuel / fuel_per_hex, LimitingResource::Fuel)
    };
    Ok(CalculatorResult {
        restrict,
        exact_range,
        complete_range: exact_range.floor() as u32,
        speed,
        total_height,
        total_burden,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculator::CalculatorError::MissingOxidizer;

    #[test]
    fn oxyrock_hydrogen() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Hydrogen,
                fuel_amount: 900.0,
                oxidizer_input: Some(OxidizerInput {
                    oxidizer: OxidizerKind::OxyRock, // 效率 2.0
                    oxidizer_amount: 400.0,          // 实际可用 800.0
                }),
                modules: vec![
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::OxidizerTank(OxidizerTankKind::LargeSolid),
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                ],
            },
        };
        let output = calculate(input).unwrap();
        let expected = CalculatorResult {
            restrict: LimitingResource::Oxidizer,
            exact_range: 14.222222,
            complete_range: 14,
            speed: 55.0 / 20.0,
            total_height: 18,
            total_burden: 20,
        };
        assert_eq!(expected.restrict, output.restrict);
        assert_eq!(expected.complete_range, output.complete_range);
        assert!((output.exact_range - expected.exact_range).abs() < 1e-5);
        assert!((output.speed - expected.speed).abs() < 1e-5);
        assert_eq!(expected.total_height, output.total_height);
        assert_eq!(expected.total_burden, output.total_burden)
    }

    #[test]
    fn liquidoxygen_hydrogen() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Hydrogen,
                fuel_amount: 800.0,
                oxidizer_input: Some(OxidizerInput {
                    oxidizer: OxidizerKind::LiquidOxygen, // 效率 4.0
                    oxidizer_amount: 200.0,               // 实际可用 800.0
                }),
                modules: vec![
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::OxidizerTank(OxidizerTankKind::Liquid),
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                ],
            },
        };
        let output = calculate(input).unwrap();
        let expected = CalculatorResult {
            restrict: LimitingResource::Balance,
            exact_range: 14.222222,
            complete_range: 14,
            speed: 55.0 / 20.0,
            total_height: 15,
            total_burden: 20,
        };
        assert_eq!(expected.restrict, output.restrict);
        assert_eq!(expected.complete_range, output.complete_range);
        assert!((output.exact_range - expected.exact_range).abs() < 1e-5);
        assert!((output.speed - expected.speed).abs() < 1e-5);
        assert_eq!(expected.total_height, output.total_height);
        assert_eq!(expected.total_burden, output.total_burden)
    }
    #[test]
    fn no_oxidizer_hydrogen() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Hydrogen,
                fuel_amount: 900.0,
                oxidizer_input: None, // 没带氧化剂！
                modules: vec![
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                ],
            },
        };
        let output = calculate(input);
        assert_eq!(output, Err(MissingOxidizer));
    }
    #[test]
    fn steam() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Steam,
                fuel_amount: 150.0,
                oxidizer_input: None, // 不需要氧化剂
                modules: vec![RocketModule::Spacefarer(
                    SpacefarerKind::SoloSpacefarerNosecone,
                )],
            },
        };
        let output = calculate(input).unwrap();
        let expected = CalculatorResult {
            restrict: LimitingResource::Fuel,
            exact_range: 10.0,
            complete_range: 10,
            speed: 27.0 / 18.0,
            total_height: 8,
            total_burden: 18,
        };
        assert_eq!(expected.restrict, output.restrict);
        assert_eq!(expected.complete_range, output.complete_range);
        assert!((output.exact_range - expected.exact_range).abs() < 1e-5);
        assert!((output.speed - expected.speed).abs() < 1e-5);
        assert_eq!(expected.total_height, output.total_height);
        assert_eq!(expected.total_burden, output.total_burden)
    }
    #[test]
    fn negative_oxizidizer() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Hydrogen,
                fuel_amount: 900.0,
                oxidizer_input: Some(OxidizerInput {
                    oxidizer: OxidizerKind::OxyRock,
                    oxidizer_amount: -200.0,
                }),
                modules: vec![
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::OxidizerTank(OxidizerTankKind::LargeSolid),
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                ],
            },
        };
        let output = calculate(input);
        assert_eq!(output, Err(CalculatorError::NegativeOxidizer))
    }
    #[test]
    fn negative_fuel() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Steam,
                fuel_amount: -200.0,
                oxidizer_input: None,
                modules: vec![RocketModule::Spacefarer(
                    SpacefarerKind::SoloSpacefarerNosecone,
                )],
            },
        };
        let output = calculate(input);
        assert_eq!(output, Err(CalculatorError::NegativeFuel));
    }

    #[test]
    fn steam_fuel_exceeds_capacity() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Steam,
                fuel_amount: 151.0,
                oxidizer_input: None,
                modules: vec![RocketModule::Spacefarer(
                    SpacefarerKind::SoloSpacefarerNosecone,
                )],
            },
        };
        let output = calculate(input);
        assert_eq!(output, Err(CalculatorError::FuelExceedsCapacity))
    }

    #[test]
    fn external_tank_fuel_exceeds_capacity() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Hydrogen,
                fuel_amount: 901.0,
                oxidizer_input: Some(OxidizerInput {
                    oxidizer: OxidizerKind::LiquidOxygen,
                    oxidizer_amount: 400.0,
                }),
                modules: vec![
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::OxidizerTank(OxidizerTankKind::LargeSolid),
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                ],
            },
        };
        let output = calculate(input);
        assert_eq!(output, Err(CalculatorError::FuelExceedsCapacity));
    }

    #[test]
    fn multiple_external_tanks_capacity() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Hydrogen,
                fuel_amount: 1800.0,
                oxidizer_input: Some(OxidizerInput {
                    oxidizer: OxidizerKind::LiquidOxygen,
                    oxidizer_amount: 450.0,
                }),
                modules: vec![
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::OxidizerTank(OxidizerTankKind::Liquid),
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                ],
            },
        };
        let output = calculate(input);
        assert_eq!(
            output.unwrap(),
            CalculatorResult {
                restrict: LimitingResource::Balance,
                exact_range: 32.0,
                complete_range: 32,
                speed: 2.2,
                total_height: 20,
                total_burden: 25,
            }
        );
    }

    #[test]
    fn liquidoxygenliquidtank() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Hydrogen,
                fuel_amount: 1800.0,
                oxidizer_input: Some(OxidizerInput {
                    oxidizer: OxidizerKind::LiquidOxygen,
                    oxidizer_amount: 450.0,
                }),
                modules: vec![
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::OxidizerTank(OxidizerTankKind::Liquid),
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                ],
            },
        };
        let output = calculate(input);
        assert_eq!(
            output.unwrap(),
            CalculatorResult {
                restrict: LimitingResource::Balance,
                exact_range: 32.0,
                complete_range: 32,
                speed: 2.2,
                total_height: 20,
                total_burden: 25,
            }
        );
    }
    #[test]
    fn incompatibleoxidizer() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Hydrogen,
                fuel_amount: 800.0,
                oxidizer_input: Some(OxidizerInput {
                    oxidizer_amount: 200.0,
                    oxidizer: OxidizerKind::LiquidOxygen,
                }),
                modules: vec![
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::OxidizerTank(OxidizerTankKind::LargeSolid),
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                ],
            },
        };
        let output = calculate(input);
        assert_eq!(output, Err(CalculatorError::IncompatibleOxidizerTank))
    }
    #[test]
    fn oxidizerexceedscapacity() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Hydrogen,
                fuel_amount: 1800.0,
                oxidizer_input: Some(OxidizerInput {
                    oxidizer: OxidizerKind::LiquidOxygen,
                    oxidizer_amount: 451.0,
                }),
                modules: vec![
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::OxidizerTank(OxidizerTankKind::Liquid),
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                ],
            },
        };
        let output = calculate(input);
        assert_eq!(output, Err(CalculatorError::OxidizerExceedsCapacity))
    }

    #[test]
    fn module_height() {
        let modules = [
            (RocketModule::FuelTank(FuelTankKind::LargeLiquid), 5),
            (RocketModule::OxidizerTank(OxidizerTankKind::SmallSolid), 2),
            (RocketModule::OxidizerTank(OxidizerTankKind::LargeSolid), 5),
            (RocketModule::OxidizerTank(OxidizerTankKind::Liquid), 2),
            (
                RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                3,
            ),
            (
                RocketModule::Spacefarer(SpacefarerKind::SpacefarerModule),
                4,
            ),
            (RocketModule::Nosecone(NoseconeKind::BasicNosecone), 2),
            (RocketModule::Nosecone(NoseconeKind::Drillcone), 4),
        ];

        for (module, expected_height) in modules {
            assert_eq!(module.height(), expected_height);
        }
    }

    #[test]
    fn module_burden() {
        let modules = [
            (RocketModule::FuelTank(FuelTankKind::LargeLiquid), 5),
            (RocketModule::OxidizerTank(OxidizerTankKind::SmallSolid), 2),
            (RocketModule::OxidizerTank(OxidizerTankKind::LargeSolid), 5),
            (RocketModule::OxidizerTank(OxidizerTankKind::Liquid), 5),
            (
                RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                3,
            ),
            (
                RocketModule::Spacefarer(SpacefarerKind::SpacefarerModule),
                6,
            ),
            (RocketModule::Nosecone(NoseconeKind::BasicNosecone), 2),
            (RocketModule::Nosecone(NoseconeKind::Drillcone), 2),
        ];

        for (module, expected_burden) in modules {
            assert_eq!(module.burden(), expected_burden);
        }
    }
    #[test]
    fn engine_height_limits() {
        let engines = [
            (EngineKind::Steam, 5, 25),
            (EngineKind::Petroleum, 5, 35),
            (EngineKind::Hydrogen, 5, 35),
        ];

        for (engine, expected_height, expected_max_height) in engines {
            let spec = engine.spec();

            assert_eq!(spec.height, expected_height);
            assert_eq!(spec.max_rocket_height, expected_max_height);
        }
    }
    #[test]
    fn rocket_exceeds_max_height() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Steam,
                fuel_amount: 150.0,
                oxidizer_input: None,
                modules: vec![
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                ],
            },
        };

        let output = calculate(input);

        assert_eq!(output, Err(CalculatorError::RocketExceedsMaxHeight));
    }

    #[test]
    fn multiple_oxidizer_tanks_capacity_and_speed() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Hydrogen,
                fuel_amount: 900.0,
                oxidizer_input: Some(OxidizerInput {
                    oxidizer: OxidizerKind::LiquidOxygen,
                    oxidizer_amount: 600.0,
                }),
                modules: vec![
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::OxidizerTank(OxidizerTankKind::Liquid),
                    RocketModule::OxidizerTank(OxidizerTankKind::Liquid),
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                ],
            },
        };
        let output = calculate(input).unwrap();
        assert!((output.speed - 2.2).abs() < 1e-5);
        assert_eq!(output.complete_range, 16);
        assert_eq!(output.restrict, LimitingResource::Fuel);
        assert_eq!(output.total_height, 17);
        assert_eq!(output.total_burden, 25);
    }

    #[test]
    fn command_module_too_less() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Steam,
                fuel_amount: 0.0,
                oxidizer_input: None,
                modules: vec![],
            },
        };

        assert_eq!(calculate(input), Err(CalculatorError::CommandModuleTooLess));
    }

    #[test]
    fn command_module_too_much() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Steam,
                fuel_amount: 0.0,
                oxidizer_input: None,
                modules: vec![
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                ],
            },
        };

        assert_eq!(calculate(input), Err(CalculatorError::CommandModuleTooMuch));
    }

    #[test]
    fn cone_module_too_less() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Steam,
                fuel_amount: 0.0,
                oxidizer_input: None,
                modules: vec![RocketModule::Spacefarer(SpacefarerKind::SpacefarerModule)],
            },
        };

        assert_eq!(calculate(input), Err(CalculatorError::ConeModuleTooLess));
    }

    #[test]
    fn cone_module_too_much() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Steam,
                fuel_amount: 0.0,
                oxidizer_input: None,
                modules: vec![
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                    RocketModule::Nosecone(NoseconeKind::BasicNosecone),
                ],
            },
        };

        assert_eq!(calculate(input), Err(CalculatorError::ConeModuleTooMuch));
    }

    #[test]
    fn spacefarer_module_with_basic_nosecone_succeeds() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Steam,
                fuel_amount: 0.0,
                oxidizer_input: None,
                modules: vec![
                    RocketModule::Spacefarer(SpacefarerKind::SpacefarerModule),
                    RocketModule::Nosecone(NoseconeKind::BasicNosecone),
                ],
            },
        };

        let output = calculate(input).expect("SpacefarerModule + BasicNosecone should succeed");

        assert_eq!(output.restrict, LimitingResource::Fuel);
        assert_eq!(output.exact_range, 0.0);
        assert_eq!(output.complete_range, 0);
        assert!((output.speed - (27.0 / 23.0)).abs() < 1e-5);
        assert_eq!(output.total_height, 11);
        assert_eq!(output.total_burden, 23);
    }

    #[test]
    fn spacefarer_test() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Hydrogen,
                fuel_amount: 800.0,
                oxidizer_input: Some(OxidizerInput {
                    oxidizer: OxidizerKind::LiquidOxygen,
                    oxidizer_amount: 200.0,
                }),
                modules: vec![
                    RocketModule::FuelTank(FuelTankKind::LargeLiquid),
                    RocketModule::OxidizerTank(OxidizerTankKind::Liquid),
                    RocketModule::Spacefarer(SpacefarerKind::SoloSpacefarerNosecone),
                ],
            },
        };
        let output = calculate(input).unwrap();
        assert_eq!(output.total_burden, 20);
        assert_eq!(output.total_height, 15);
    }
    #[test]
    fn spacefarer_module_with_drillcone_succeeds() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Steam,
                fuel_amount: 100.0,
                oxidizer_input: None,
                modules: vec![
                    RocketModule::Spacefarer(SpacefarerKind::SpacefarerModule),
                    RocketModule::Nosecone(NoseconeKind::Drillcone),
                ],
            },
        };
        let output = calculate(input).unwrap();
        assert_eq!(output.total_height, 13);
        assert_eq!(output.total_burden, 23);
        assert!((output.speed - (27f32 / 23f32)).abs() < 1e-5)
    }
}
