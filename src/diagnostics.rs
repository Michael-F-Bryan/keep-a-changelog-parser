use codespan::FileId;
use codespan_reporting::diagnostic::{Diagnostic, Severity};

#[derive(Debug, Default, Clone)]
pub struct Diagnostics {
    diags: Vec<Diagnostic<FileId>>,
}

impl Diagnostics {
    pub fn new() -> Self { Self { diags: Vec::new() } }

    pub fn diagnostics(&self) -> &[Diagnostic<FileId>] { &self.diags }

    pub fn contains_at_least(&self, severity: Severity) -> bool {
        self.diagnostics()
            .iter()
            .any(|diag| diag.severity >= severity)
    }
}
