mod changelog;

pub use changelog::{Changelog, Changes, Item, Release};

use codespan::FileId;
use codespan_reporting::diagnostic::Diagnostic;

pub fn parse<F>(_file_id: FileId, _src: &str, _on_diagnostic: F) -> Changelog
where
    F: FnMut(Diagnostic<FileId>),
{
    unimplemented!()
}
