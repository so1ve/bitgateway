#[derive(Clone, Debug, PartialEq)]
pub struct OnlineInfo {
    pub username: String,
    pub ip: String,
    pub used_flux: String,
    pub used_duration: String,
    pub balance: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SessionPhase {
    #[default]
    Initializing,
    Offline,
    LoggingIn {
        automatic: bool,
    },
    Online,
    LoggingOut,
}

impl SessionPhase {
    pub const fn is_busy(&self) -> bool {
        matches!(self, Self::LoggingIn { .. } | Self::LoggingOut)
    }
}
