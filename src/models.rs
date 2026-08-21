// 燃料相关
#[derive(Debug, Clone, Copy)]
pub enum FuelStorage {
    Internal { capacity: f32 },
    ExternalTank,
}

#[derive(Debug, Clone, Copy)]
pub enum FuelTankKind {
    LargeLiquid,
}

#[derive(Debug, Clone, Copy)]
pub struct FuelTankSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub capacity: f32,
    pub burden: u32,
    pub height: u32,
}

impl FuelTankKind {
    pub fn spec(self) -> FuelTankSpec {
        match self {
            FuelTankKind::LargeLiquid => FuelTankSpec {
                id: "LargeLiquidFuelTank",
                name: "大型液体燃料舱",
                capacity: 900.0,
                burden: 5,
                height: 5,
            },
        }
    }
}

// 引擎相关
#[derive(Debug, Clone, Copy)]
pub struct EngineSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub fuel_name: &'static str,
    pub fuel_per_hex: f32,

    pub fuel_storage: FuelStorage,

    pub requires_oxidizer: bool,

    pub height: u32,
    pub max_rocket_height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineKind {
    Hydrogen,
    Steam,
    Petroleum,
}
impl EngineKind {
    pub const ALL: [EngineKind; 3] = [
        EngineKind::Hydrogen,
        EngineKind::Petroleum,
        EngineKind::Steam,
    ];
    pub fn spec(self) -> EngineSpec {
        match self {
            EngineKind::Hydrogen => EngineSpec {
                id: "HydrogenEngineCluster",
                name: "液氢引擎",
                fuel_name: "液氢",
                fuel_per_hex: 56.25,
                fuel_storage: FuelStorage::ExternalTank,
                requires_oxidizer: true,
                height: 5,
                max_rocket_height: 35,
            },
            EngineKind::Steam => EngineSpec {
                id: "SteamEngine",
                name: "蒸汽引擎",
                fuel_name: "蒸汽",
                fuel_per_hex: 15.0,
                fuel_storage: FuelStorage::Internal { capacity: 150.0 },
                requires_oxidizer: false, // 蒸汽引擎不需要氧化剂
                height: 5,
                max_rocket_height: 25,
            },
            EngineKind::Petroleum => EngineSpec {
                id: "PetroleumEngine",
                name: "石油引擎",
                fuel_name: "石油",
                fuel_per_hex: 90.0,
                fuel_storage: FuelStorage::ExternalTank,
                requires_oxidizer: true,
                height: 5,
                max_rocket_height: 35,
            },
        }
    }
}

// 氧化剂相关
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
    pub const ALL: [OxidizerKind; 3] = [
        OxidizerKind::LiquidOxygen,
        OxidizerKind::OxyRock,
        OxidizerKind::Fertilizer,
    ];
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
    pub fn storage_kind(self) -> OxidizerStorageKind {
        match self {
            OxidizerKind::Fertilizer => OxidizerStorageKind::Solid,
            OxidizerKind::OxyRock => OxidizerStorageKind::Solid,
            OxidizerKind::LiquidOxygen => OxidizerStorageKind::Liquid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OxidizerStorageKind {
    Liquid,
    Solid,
}

#[derive(Debug, Clone, Copy)]
pub struct OxidizerTankSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub capacity: f32,
    pub burden: u32,
    pub height: u32,

    pub storage_kind: OxidizerStorageKind,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OxidizerTankKind {
    SmallSolid,
    LargeSolid,
    Liquid,
}
impl OxidizerTankKind {
    pub fn spec(self) -> OxidizerTankSpec {
        match self {
            OxidizerTankKind::SmallSolid => OxidizerTankSpec {
                id: "SmallSolidOxidizerTank",
                name: "小型固体氧化剂舱",
                capacity: 450.0,
                burden: 2,
                height: 2,
                storage_kind: OxidizerStorageKind::Solid,
            },
            OxidizerTankKind::LargeSolid => OxidizerTankSpec {
                id: "LargeSolidOxidizerTank",
                name: "大型固体氧化剂舱",
                capacity: 900.0,
                burden: 5,
                height: 5,
                storage_kind: OxidizerStorageKind::Solid,
            },

            OxidizerTankKind::Liquid => OxidizerTankSpec {
                id: "LiquidOxidizerTank",
                name: "液体氧化剂舱",
                capacity: 450.0,
                burden: 5,
                height: 2,
                storage_kind: OxidizerStorageKind::Liquid,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OxidizerInput {
    pub oxidizer: OxidizerKind,
    pub oxidizer_amount: f32,
}

//火箭相关
pub struct RocketInput {
    pub engine: EngineKind,
    pub fuel_amount: f32,
    pub oxidizer_input: Option<OxidizerInput>,
    pub modules: Vec<RocketModule>,
}

#[derive(Debug, Clone, Copy)]
pub enum RocketModule {
    FuelTank(FuelTankKind),
    OxidizerTank(OxidizerTankKind),
}

impl RocketModule {
    pub fn height(self) -> u32 {
        match self {
            RocketModule::FuelTank(tank) => tank.spec().height,
            RocketModule::OxidizerTank(tank) => tank.spec().height,
        }
    }
    pub fn burden(self) -> u32 {
        match self {
            RocketModule::FuelTank(tank) => tank.spec().burden,
            RocketModule::OxidizerTank(tank) => tank.spec().burden,
        }
    }
}
