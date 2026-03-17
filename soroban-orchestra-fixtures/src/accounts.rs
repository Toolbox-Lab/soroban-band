use soroban_sdk::Address;
use soroban_orchestra_core::OrchestraEnv;

pub struct AccountFixture {
    pub alias: String,
    pub address: Address,
}

pub struct Personas;

impl Personas {
    pub fn whale(env: &OrchestraEnv) -> AccountFixture {
        AccountFixture {
            alias: "whale".into(),
            address: Address::generate(&env.env),
        }
    }

    pub fn dust(env: &OrchestraEnv) -> AccountFixture {
        AccountFixture {
            alias: "dust".into(),
            address: Address::generate(&env.env),
        }
    }
}
