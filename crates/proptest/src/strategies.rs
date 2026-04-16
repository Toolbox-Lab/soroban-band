use proptest::strategy::Strategy;
use soroban_sdk::Address;

pub struct SorobanStrategies;

impl SorobanStrategies {
    pub fn address() -> impl Strategy<Value = Address> {
        // Mock implementation of address generation
        proptest::strategy::Just(Address::from_string(&soroban_sdk::String::from_str(&soroban_sdk::Env::default(), "GBRX2Q...")))
    }

    pub fn amount() -> impl Strategy<Value = i128> {
        0..1_000_000_000_000_000_000i128 // Mock valid ranges
    }
}
