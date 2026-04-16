/// Layer 4: Coverage & Reporting
/// 
/// Handles interaction tracking and report generation:
/// - Cross-contract call graph tracking
/// - Coverage metrics
/// - HTML report generation

pub mod coverage;
pub mod report;

pub use coverage::CoverageTracker;
pub use report::HtmlReport;
