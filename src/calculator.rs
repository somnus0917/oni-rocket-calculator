use crate::models::*;
pub struct CalculatorInput {
    pub rocket: RocketInput,
}
#[derive(Debug, PartialEq)]
pub enum LimitingResource {
    Fuel,
    Oxidizer,
    Balance,
}
#[derive(Debug, PartialEq)]
pub struct CalculatorResult {
    pub restrict: LimitingResource,
    pub exact_range: f32,
    pub complete_range: u32,
}
#[derive(Debug, PartialEq)]
pub enum CalculatorError {
    NegativeFuel,
    MissingOxidizer,
    NegativeOxidizer,
}
pub fn calculate(input: CalculatorInput) -> Result<CalculatorResult, CalculatorError> {
    let rocket = input.rocket;
    let fuel = rocket.fuel_amount;
    if fuel < 0.0 {
        return Err(CalculatorError::NegativeFuel);
    }
    let fuel_per_hex = rocket.engine.spec().fuel_per_hex;
    let (exact_range, restrict) = if rocket.engine.spec().requires_oxidizer {
        let oxi = rocket
            .oxidizerinput
            .ok_or(CalculatorError::MissingOxidizer)?;
        if oxi.oxidizer_amount < 0.0 {
            return Err(CalculatorError::NegativeOxidizer);
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
                fuel_amount: 1000.0,
                oxidizerinput: Some(OxidizerInput {
                    oxidizer: OxidizerKind::OxyRock, // 效率 2.0
                    oxidizer_amount: 400.0,          // 实际可用 800.0
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
            rocket: RocketInput {
                engine: EngineKind::Hydrogen,
                fuel_amount: 800.0,
                oxidizerinput: Some(OxidizerInput {
                    oxidizer: OxidizerKind::LiquidOxygen, // 效率 4.0
                    oxidizer_amount: 200.0,               // 实际可用 800.0
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
            rocket: RocketInput {
                engine: EngineKind::Hydrogen,
                fuel_amount: 900.0,
                oxidizerinput: None, // 没带氧化剂！
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
                fuel_amount: 500.0,
                oxidizerinput: None, // 不需要氧化剂
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
    #[test]
    fn negative_oxizidizer() {
        let input = CalculatorInput {
            rocket: RocketInput {
                engine: EngineKind::Hydrogen,
                fuel_amount: 500.0,
                oxidizerinput: Some(OxidizerInput {
                    oxidizer: OxidizerKind::OxyRock,
                    oxidizer_amount: -200.0,
                }),
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
                oxidizerinput: None,
            },
        };
        let output = calculate(input);
        assert_eq!(output, Err(CalculatorError::NegativeFuel));
    }
}
