#[derive(Debug, Clone, Copy)]
pub struct EngineSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub fuel_name: &'static str,
    pub fuel_per_hex: f32,
    pub requires_oxidizer: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineKind {
    Hydrogen,
    Steam,
}
impl EngineKind {
    pub fn spec(self) -> EngineSpec {
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
pub struct OxidizerSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub efficiency: f32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OxidizerKind {
    Fertilizer,
    OxyRock,
    LiquidOxygen,
}
impl OxidizerKind {
    pub const fn spec(self) -> OxidizerSpec {
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
pub struct OxidizerInput {
    pub oxidizer: OxidizerKind,
    pub oxidizer_amount: f32,
}
pub struct RocketInput {
    pub engine: EngineKind,
    pub fuel_amount: f32,
    pub oxidizerinput: Option<OxidizerInput>,
}
