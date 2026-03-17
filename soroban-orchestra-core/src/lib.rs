/// Layer 1: Orchestration Core
/// 
/// This crate provides the environment builder, auth simulation, 
/// and lifecycle management for multi-contract Soroban tests.

pub mod env;
pub mod auth;
pub mod lifecycle;

pub use env::{OrchestraEnv, OrchestraEnvBuilder};
pub use auth::{AuthTrace, AuthNode, AuthResult, AuthDenialReason};
