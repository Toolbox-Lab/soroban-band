use soroban_band_core::BandEnv;
use soroban_sdk::{Address, String};
use soroban_sdk::testutils::Address as _;

pub struct TokenFixture {
    pub address: Address,
}

impl TokenFixture {
    pub fn new(env: &BandEnv, name: String, symbol: String, decimals: u32) -> Self {
        // Mock deploy and setup token
        TokenFixture {
            address: Address::generate(&env.env),
        }
    }
}

pub struct Sep41Token;
