use soroban_sdk::{Address, Symbol, Val};

/// A single node in the authorization tree.
#[derive(Debug, Clone)]
pub struct AuthNode {
    pub contract_alias: String,
    pub contract_address: Address,
    pub function_name: Symbol,
    pub auth_args: Option<std::vec::Vec<Val>>,
    pub principal: Address,
    pub result: AuthResult,
    pub children: std::vec::Vec<AuthNode>,
}

#[derive(Debug, Clone)]
pub enum AuthResult {
    Approved,
    Denied(AuthDenialReason),
}

#[derive(Debug, Clone)]
pub enum AuthDenialReason {
    MissingAuth { principal: Address },
    ArgMismatch { expected: std::vec::Vec<Val>, got: std::vec::Vec<Val> },
    AlreadyConsumed,
    WrongContractOrFunction {
        expected_contract: Address,
        expected_fn: Symbol,
    },
}

#[derive(Debug, Clone)]
pub struct AuthTrace {
    pub root: AuthNode,
    pub all_nodes: std::vec::Vec<AuthNode>,
    pub unused_auth_entries: std::vec::Vec<Address>,
}

impl AuthTrace {
    pub fn print_tree(&self) {
        // Implementation for ASCII tree printing
    }

    pub fn assert_all_approved(&self) {
        // Implementation for asserting failure with structured error
    }
}
