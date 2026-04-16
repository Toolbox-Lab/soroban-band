use soroban_sdk::Address;
use soroban_band_core::BandEnv;

pub struct AccountFixture {
    pub alias: String,
    pub address: Address,
}

pub struct Personas;

impl Personas {
    pub fn whale(env: &BandEnv) -> AccountFixture {
        AccountFixture {
            alias: "whale".into(),
            address: Address::generate(&env.env),
        }
    }

    pub fn dust(env: &BandEnv) -> AccountFixture {
        AccountFixture {
            alias: "dust".into(),
            address: Address::generate(&env.env),
        }
    }
}
