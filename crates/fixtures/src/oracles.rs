use soroban_sdk::Symbol;

pub trait OracleInterface {
    fn price(&self, base: Symbol, quote: Symbol) -> Option<PriceData>;
    fn on_ledger_advance(&mut self, new_timestamp: u64, new_sequence: u32);
}

pub struct PriceData {
    pub price: i128,
    pub decimals: u32,
    pub timestamp: u64,
    pub confidence: Option<i128>,
}

pub struct SimplePriceFeed;

impl OracleInterface for SimplePriceFeed {
    fn price(&self, _base: Symbol, _quote: Symbol) -> Option<PriceData> {
        None
    }
    
    fn on_ledger_advance(&mut self, _new_timestamp: u64, _new_sequence: u32) {
    }
}
