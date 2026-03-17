use crate::coverage::CoverageTracker;

pub struct HtmlReport;

impl HtmlReport {
    pub fn generate(tracker: &CoverageTracker) -> String {
        // Generate self-contained HTML via Askama template
        "<html><body><h1>Coverage Report</h1></body></html>".to_string()
    }
}
