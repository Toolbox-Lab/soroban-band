/// Layer 1: Bandtion Core
/// 
/// This crate provides the environment builder, auth simulation, 
/// and lifecycle management for multi-contract Soroban tests.

pub mod env;
pub mod auth;
pub mod lifecycle;

pub use env::{BandEnv, BandEnvBuilder};
pub use auth::{AuthTrace, AuthNode, AuthResult, AuthDenialReason};
