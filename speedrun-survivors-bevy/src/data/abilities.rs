use strum::EnumIter;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter)]
pub enum AbilityType {
    OrcaChopper,
    RugPull,
    ShitcoinCluster,
    WhaleDump,
}

impl AbilityType {
    pub fn select_randomly(amount: usize) -> Vec<AbilityType> {
        eprintln!("TODO: Select abilities randomly");

        vec![
            AbilityType::OrcaChopper,
            AbilityType::RugPull,
            AbilityType::ShitcoinCluster,
            AbilityType::WhaleDump,
        ]
    }
}

impl std::fmt::Display for AbilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbilityType::OrcaChopper => write!(f, "Orca Chopper"),
            AbilityType::RugPull => write!(f, "Rug Pull"),
            AbilityType::ShitcoinCluster => write!(f, "Shitcoin Cluster"),
            AbilityType::WhaleDump => write!(f, "Whale Dump"),
        }
    }
}
