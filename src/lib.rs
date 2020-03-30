mod diagnostics;
mod parser;

pub use diagnostics::Diagnostics;
pub use parser::parse;

use chrono::NaiveDate;
use codespan::Span;
use pulldown_cmark::Event;
use semver::Version;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Changelog {
    pub releases: Vec<Release>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Release {
    pub version: Option<Version>,
    pub date: Option<NaiveDate>,
    pub link: Option<String>,
    pub changes: Changes,
    pub span: Span,
}

#[derive(Debug, Default, Clone, PartialEq)]
#[non_exhaustive]
pub struct Changes {
    pub added: Vec<Item>,
    pub changed: Vec<Item>,
    pub fixed: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Item {
    pub body: Vec<Event<'static>>,
    pub children: Vec<Item>,
    pub span: Span,
}
