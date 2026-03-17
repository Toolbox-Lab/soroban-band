use soroban_sdk::{Address, Symbol, Val, Vec};

/// A single node in the authorization tree.
#[derive(Debug, Clone, serde::Serialize)]
pub struct AuthNode {
    pub contract_alias: String,
    pub contract_address: Address,
    pub function_name: Symbol,
    pub auth_args: Option<Vec<Val>>,
    pub principal: Address,
    pub result: AuthResult,
    pub children: Vec<AuthNode>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum AuthResult {
    Approved,
    Denied(AuthDenialReason),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum AuthDenialReason {
    MissingAuth { principal: Address },
    ArgMismatch { expected: Vec<Val>, got: Vec<Val> },
    AlreadyConsumed,
    WrongContractOrFunction {
        expected_contract: Address,
        expected_fn: Symbol,
    },
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AuthTrace {
    pub root: AuthNode,
    pub all_nodes: Vec<AuthNode>,
    pub unused_auth_entries: Vec<Address>,
}

impl AuthTrace {
    pub fn print_tree(&self) {
        // Implementation for ASCII tree printing
    }

    pub fn assert_all_approved(&self) {
        // Implementation for asserting failure with structured error
    }
}
