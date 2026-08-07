#[derive(Debug, Clone, Copy)]
struct EngineSpec {
    id: &'static str,
    name: &'static str,
    fuel_name: &'static str,
    fuel_per_hex: f32,
    requires_oxidizer: bool,
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
                fuel_name: "液氢",
                fuel_per_hex: 56.25,
                requires_oxidizer: true,
            },
            EngineKind::Steam => EngineSpec {
                id: "SteamEngine",
                name: "蒸汽引擎",
                fuel_name: "蒸汽",
                fuel_per_hex: 20.0,
                requires_oxidizer: false, // 蒸汽引擎不需要氧化剂
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
    Oxyrock,
    Liquidoxygen,
}
impl OxidizerKind {
    const fn spec(self) -> OxidizerSpec {
        match self {
            OxidizerKind::Fertilizer => OxidizerSpec {
                id: "fertilizer",
                name: "肥料",
                efficiency: 1.0,
            },
            OxidizerKind::Oxyrock => OxidizerSpec {
                id: "oxyrock",
                name: "氧石",
                efficiency: 2.0,
            },
            OxidizerKind::Liquidoxygen => OxidizerSpec {
                id: "liquidoxygen",
                name: "液氧",
                efficiency: 4.0,
            },
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct OxidizerInput {
    oxidizer: OxidizerKind,
    oxidizer_amount: f32,
}
struct RocketSpec {
    engine: EngineSpec,
    fuel_amount: f32,
    oxidizerinput: Option<OxidizerInput>,
}
struct CalculatorInput {
    rocket: RocketSpec,
}
#[derive(Debug)]
enum RestrictFactor {
    FUEL,
    OXIDIZER,
    BALANCE,
}
#[derive(Debug)]
struct CalculatorResult {
    restrict: RestrictFactor,
    exact_range: f32,
    complete_range: u32,
}
#[derive(Debug)]
enum CalculatorError {
    NegativeFuel,
    MissingOxidizer,
    NegativeOxidizer,
}
fn calculate(input: CalculatorInput) -> Result<CalculatorResult, CalculatorError> {
    let rocket = input.rocket;
    let fuel = rocket.fuel_amount;
    let fuel_per_hex = rocket.engine.fuel_per_hex;
    let (exact_range, restrict) = if rocket.engine.requires_oxidizer {
        let oxi = rocket
            .oxidizerinput
            .ok_or(CalculatorError::MissingOxidizer)?;
        let effective_oxi = oxi.oxidizer_amount * oxi.oxidizer.spec().efficiency;
        let range = fuel.min(effective_oxi) / fuel_per_hex;
        let res = match fuel - effective_oxi {
            x if x > 0.0 => RestrictFactor::OXIDIZER,
            x if x == 0.0 => RestrictFactor::BALANCE,
            _ => RestrictFactor::FUEL,
        };
        (range, res)
    } else {
        (fuel / fuel_per_hex, RestrictFactor::FUEL)
    };
    Ok(CalculatorResult {
        restrict,
        exact_range,
        complete_range: exact_range as u32,
    })
}
fn main() {
    println!("=== 缺氧 (Oxygen Not Included) 火箭航程计算器测试 ===\n");

    // 测试 1：液氢引擎 + 氧石，燃料多于氧化剂，应该提示 OXIDIZER 受限
    let input1 = CalculatorInput {
        rocket: RocketSpec {
            engine: EngineKind::Hydrogen.spec(),
            fuel_amount: 1000.0,
            oxidizerinput: Some(OxidizerInput {
                oxidizer: OxidizerKind::Oxyrock, // 效率 2.0
                oxidizer_amount: 400.0,          // 实际可用 800.0
            }),
        },
    };
    println!("测试 1 (燃料: 1000, 氧化石: 400):");
    println!("{:#?}\n", calculate(input1));

    // 测试 2：液氢引擎 + 液氧，完美平衡，应该提示 BALANCE
    let input2 = CalculatorInput {
        rocket: RocketSpec {
            engine: EngineKind::Hydrogen.spec(),
            fuel_amount: 800.0,
            oxidizerinput: Some(OxidizerInput {
                oxidizer: OxidizerKind::Liquidoxygen, // 效率 4.0
                oxidizer_amount: 200.0,               // 实际可用 800.0
            }),
        },
    };
    println!("测试 2 (燃料: 800, 液氧: 200 - 完美平衡):");
    println!("{:#?}\n", calculate(input2));

    // 测试 3：液氢引擎忘记带氧化剂，应该报错 MissingOxidizer
    let input3 = CalculatorInput {
        rocket: RocketSpec {
            engine: EngineKind::Hydrogen.spec(),
            fuel_amount: 900.0,
            oxidizerinput: None, // 没带氧化剂！
        },
    };
    println!("测试 3 (需要氧化剂却没带):");
    println!("{:#?}\n", calculate(input3));

    // 测试 4：蒸汽引擎，不需要氧化剂
    let input4 = CalculatorInput {
        rocket: RocketSpec {
            engine: EngineKind::Steam.spec(),
            fuel_amount: 500.0,
            oxidizerinput: None, // 不需要氧化剂
        },
    };
    println!("测试 4 (蒸汽引擎，不需要氧化剂):");
    println!("{:#?}", calculate(input4));
}
