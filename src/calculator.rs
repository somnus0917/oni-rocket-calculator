#[derive(Debug, Clone, Copy)]
struct EngineSpec {
    id: &'static str,
    name: &'static str,
    Fuel_name: &'static str,
    Fuel_per_hex: f32,
    requires_Oxidizer: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EngineKind {
    Hydrogen,
    Steam,
}
impl EngineKind {
    fn spec(self) -> EngineSpec {
        match self {
            EngineKind::Hydrogen => EngineSpec {
                id: "HydrogenEngineCluster",
                name: "液氢引擎",
                Fuel_name: "液氢",
                Fuel_per_hex: 56.25,
                requires_Oxidizer: true,
            },
            EngineKind::Steam => EngineSpec {
                id: "SteamEngine",
                name: "蒸汽引擎",
                Fuel_name: "蒸汽",
                Fuel_per_hex: 20.0,
                requires_Oxidizer: false, // 蒸汽引擎不需要氧化剂
            },
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct OxidizerSpec {
    id: &'static str,
    name: &'static str,
    efficiency: f32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OxidizerKind {
    Fertilizer,
    OxyRock,
    LiquidOxygen,
}
impl OxidizerKind {
    const fn spec(self) -> OxidizerSpec {
        match self {
            OxidizerKind::Fertilizer => OxidizerSpec {
                id: "fertilizer",
                name: "肥料",
                efficiency: 1.0,
            },
            OxidizerKind::OxyRock => OxidizerSpec {
                id: "OxyRock",
                name: "氧石",
                efficiency: 2.0,
            },
            OxidizerKind::LiquidOxygen => OxidizerSpec {
                id: "LiquidOxygen",
                name: "液氧",
                efficiency: 4.0,
            },
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct OxidizerInput {
    Oxidizer: OxidizerKind,
    Oxidizer_amount: f32,
}
struct RocketSpec {
    engine: EngineSpec,
    Fuel_amount: f32,
    Oxidizerinput: Option<OxidizerInput>,
}
pub struct CalculatorInput {
    rocket: RocketSpec,
}
#[derive(Debug, PartialEq)]
enum LimitingResource {
    Fuel,
    Oxidizer,
    Balance,
}
#[derive(Debug, PartialEq)]
pub struct CalculatorResult {
    restrict: LimitingResource,
    exact_range: f32,
    complete_range: u32,
}
#[derive(Debug, PartialEq)]
pub enum CalculatorError {
    NegativeFuel,
    MissingOxidizer,
    NegativeOxidizer,
}
pub fn calculate(input: CalculatorInput) -> Result<CalculatorResult, CalculatorError> {
    let rocket = input.rocket;
    let Fuel = rocket.Fuel_amount;
    let Fuel_per_hex = rocket.engine.Fuel_per_hex;
    let (exact_range, restrict) = if rocket.engine.requires_Oxidizer {
        let oxi = rocket
            .Oxidizerinput
            .ok_or(CalculatorError::MissingOxidizer)?;
        let effective_oxi = oxi.Oxidizer_amount * oxi.Oxidizer.spec().efficiency;
        let range = Fuel.min(effective_oxi) / Fuel_per_hex;
        let res = match Fuel - effective_oxi {
            x if x > 0.0 => LimitingResource::Oxidizer,
            x if x == 0.0 => LimitingResource::Balance,
            _ => LimitingResource::Fuel,
        };
        (range, res)
    } else {
        (Fuel / Fuel_per_hex, LimitingResource::Fuel)
    };
    Ok(CalculatorResult {
        restrict,
        exact_range,
        complete_range: exact_range as u32,
    })
}

#[cfg(test)]
mod tests {
    use crate::calculator::CalculatorError::MissingOxidizer;

    use super::*;

    #[test]
    fn oxyrock_hydrogen() {
        let input = CalculatorInput {
            rocket: RocketSpec {
                engine: EngineKind::Hydrogen.spec(),
                Fuel_amount: 1000.0,
                Oxidizerinput: Some(OxidizerInput {
                    Oxidizer: OxidizerKind::OxyRock, // 效率 2.0
                    Oxidizer_amount: 400.0,          // 实际可用 800.0
                }),
            },
        };
        let output = calculate(input).unwrap();
        let expected = CalculatorResult {
            restrict: LimitingResource::Oxidizer,
            exact_range: 14.222222,
            complete_range: 14,
        };
        assert_eq!(expected.restrict, output.restrict);
        assert_eq!(expected.complete_range, output.complete_range);
        assert!((output.exact_range - expected.exact_range).abs() < 1e-5)
    }

    #[test]
    fn liquidoxygen_hydrogen() {
        let input = CalculatorInput {
            rocket: RocketSpec {
                engine: EngineKind::Hydrogen.spec(),
                Fuel_amount: 800.0,
                Oxidizerinput: Some(OxidizerInput {
                    Oxidizer: OxidizerKind::LiquidOxygen, // 效率 4.0
                    Oxidizer_amount: 200.0,               // 实际可用 800.0
                }),
            },
        };
        let output = calculate(input).unwrap();
        let expected = CalculatorResult {
            restrict: LimitingResource::Balance,
            exact_range: 14.222222,
            complete_range: 14,
        };
        assert_eq!(expected.restrict, output.restrict);
        assert_eq!(expected.complete_range, output.complete_range);
        assert!((output.exact_range - expected.exact_range).abs() < 1e-5)
    }
    #[test]
    fn no_oxidizer_hydrogen() {
        let input = CalculatorInput {
            rocket: RocketSpec {
                engine: EngineKind::Hydrogen.spec(),
                Fuel_amount: 900.0,
                Oxidizerinput: None, // 没带氧化剂！
            },
        };
        let output = calculate(input);
        assert_eq!(output, Err(MissingOxidizer));
    }
    #[test]
    fn steam() {
        let input = CalculatorInput {
            rocket: RocketSpec {
                engine: EngineKind::Steam.spec(),
                Fuel_amount: 500.0,
                Oxidizerinput: None, // 不需要氧化剂
            },
        };
        let output = calculate(input).unwrap();
        let expected = CalculatorResult {
            restrict: LimitingResource::Fuel,
            exact_range: 25.0,
            complete_range: 25,
        };
        assert_eq!(expected.restrict, output.restrict);
        assert_eq!(expected.complete_range, output.complete_range);
        assert!((output.exact_range - expected.exact_range).abs() < 1e-5)
    }
}
