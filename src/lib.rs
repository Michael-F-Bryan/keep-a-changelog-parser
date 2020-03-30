mod diagnostics;

pub use diagnostics::Diagnostics;

use chrono::NaiveDate;
use codespan::{FileId, Span};
use codespan_reporting::diagnostic::Diagnostic;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Tag};
use semver::Version;

#[derive(Debug, Clone, PartialEq)]
pub struct Changelog {
    pub releases: Vec<Release>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Release {
    pub version: Option<Version>,
    pub date: Option<NaiveDate>,
    pub link: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub body: Vec<Event<'static>>,
    pub children: Vec<Item>,
    pub span: Span,
}

pub fn parse<F>(file_id: FileId, src: &str, on_diagnostic: F) -> Changelog
where
    F: FnMut(Diagnostic<FileId>),
{
    let overall_span = Span::from_str(src);
    unimplemented!()
}

pub(crate) fn owned_event(ev: Event<'_>) -> Event<'static> {
    match ev {
        Event::Start(tag) => Event::Start(owned_tag(tag)),
        Event::End(tag) => Event::End(owned_tag(tag)),
        Event::Text(s) => Event::Text(owned_cow_str(s)),
        Event::Code(s) => Event::Code(owned_cow_str(s)),
        Event::Html(s) => Event::Html(owned_cow_str(s)),
        Event::FootnoteReference(s) => {
            Event::FootnoteReference(owned_cow_str(s))
        },
        Event::SoftBreak => Event::SoftBreak,
        Event::HardBreak => Event::HardBreak,
        Event::Rule => Event::Rule,
        Event::TaskListMarker(t) => Event::TaskListMarker(t),
    }
}

pub(crate) fn owned_cow_str(s: CowStr<'_>) -> CowStr<'static> {
    match s {
        CowStr::Borrowed(_) => CowStr::from(s.into_string()),
        CowStr::Boxed(boxed) => CowStr::Boxed(boxed),
        CowStr::Inlined(inlined) => CowStr::Inlined(inlined),
    }
}

pub(crate) fn owned_tag(tag: Tag<'_>) -> Tag<'static> {
    match tag {
        Tag::Paragraph => Tag::Paragraph,
        Tag::Heading(h) => Tag::Heading(h),
        Tag::BlockQuote => Tag::BlockQuote,
        Tag::CodeBlock(CodeBlockKind::Indented) => {
            Tag::CodeBlock(CodeBlockKind::Indented)
        },
        Tag::CodeBlock(CodeBlockKind::Fenced(s)) => {
            Tag::CodeBlock(CodeBlockKind::Fenced(owned_cow_str(s)))
        },
        Tag::List(u) => Tag::List(u),
        Tag::Item => Tag::Item,
        Tag::FootnoteDefinition(s) => Tag::FootnoteDefinition(owned_cow_str(s)),
        Tag::Table(alignment) => Tag::Table(alignment),
        Tag::TableHead => Tag::TableHead,
        Tag::TableRow => Tag::TableRow,
        Tag::TableCell => Tag::TableCell,
        Tag::Emphasis => Tag::Emphasis,
        Tag::Strong => Tag::Strong,
        Tag::Strikethrough => Tag::Strikethrough,
        Tag::Link(t, url, title) => {
            Tag::Link(t, owned_cow_str(url), owned_cow_str(title))
        },
        Tag::Image(t, url, alt) => {
            Tag::Image(t, owned_cow_str(url), owned_cow_str(alt))
        },
    }
}
