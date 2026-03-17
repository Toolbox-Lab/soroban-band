/// Layer 3: Property-Based Testing Engine
/// 
/// Provides strategies and stateful fuzzing for multi-contract systems:
/// - Cross-contract strategy generators
/// - Stateful property testing
/// - Invariant definitions
/// - Chaos Monkey mode

pub mod strategies;
pub mod stateful;
pub mod invariants;
pub mod chaos;

pub use strategies::SorobanStrategies;
pub use stateful::StateMachineRunner;
pub use chaos::ChaosMonkey;
