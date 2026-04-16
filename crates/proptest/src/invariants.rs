use soroban_band_core::BandEnv;

pub type InvariantFn = Box<dyn Fn(&BandEnv) -> Result<(), InvariantViolation>>;

pub struct InvariantViolation {
    pub msg: String,
}

impl InvariantViolation {
    pub fn new(msg: &str) -> Self {
        Self { msg: msg.into() }
    }
}
