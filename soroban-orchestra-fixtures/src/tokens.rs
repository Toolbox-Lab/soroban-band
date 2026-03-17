use soroban_orchestra_core::OrchestraEnv;
use soroban_sdk::{Address, String};

pub struct TokenFixture {
    pub address: Address,
}

impl TokenFixture {
    pub fn new(env: &OrchestraEnv, name: String, symbol: String, decimals: u32) -> Self {
        // Mock deploy and setup token
        TokenFixture {
            address: Address::generate(&env.env),
        }
    }
}

pub struct Sep41Token;
