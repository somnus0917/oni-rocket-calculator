#[derive(Debug, Clone, Copy)]
struct EngineSpec {
    id: &'static str,
    name: &'static str,
    fuel_name: &'static str,
    fuel_per_hex: f32,
    requires_oxidizer: bool,
}
enum EngineKind {
    Hydrogen,
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
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct OxidizerSpec {
    id: &'static str,
    name: &'static str,
    efficiency: f32,
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
enum OxidizerKind {
    Fertilizer,
    Oxyrock,
    Liquidoxygen,
}
struct OxidizerInput {
    oxidizer: OxidizerKind,
    oxidizer_amount: f32,
}
struct RocketSpec {
    engine: EngineSpec,
    fuel_amount: i32,
    oxidizerinput: Option<OxidizerInput>,
}
struct CalculatorInput {
    rocket: RocketSpec,
}
enum RestrictFactor {
    FUEL,
    OXIDIZER,
    BALANCE,
}
struct CalculatorResult {
    restrict: RestrictFactor,
    range: i32,
}

fn main() {
    println!("Hello, world!");
}
