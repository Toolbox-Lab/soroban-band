use soroban_orchestra_core::OrchestraEnv;

pub type InvariantFn = Box<dyn Fn(&OrchestraEnv) -> Result<(), InvariantViolation>>;

pub struct InvariantViolation {
    pub msg: String,
}

impl InvariantViolation {
    pub fn new(msg: &str) -> Self {
        Self { msg: msg.into() }
    }
}
