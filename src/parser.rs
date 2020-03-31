use crate::{Changelog, Release};
use codespan::{FileId, Span};
use codespan_reporting::diagnostic::Diagnostic;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Tag};

pub fn parse<F>(file_id: FileId, src: &str, mut on_diagnostic: F) -> Changelog
where
    F: FnMut(Diagnostic<FileId>),
{
    let mut state = State {
        file_id,
        releases: Vec::new(),
    };
    let mut parser = Parser::default();

    for (event, range) in pulldown_cmark::Parser::new(src).into_offset_iter() {
        let span = Span::new(range.start as u32, range.end as u32);
        let event = owned_event(event);
        parser = parser.process(event, span, &mut state, &mut on_diagnostic);
    }

    parser.flush(&mut state, &mut on_diagnostic);

    Changelog {
        releases: state.releases,
        span: Span::from_str(src),
    }
}

struct State {
    file_id: FileId,
    releases: Vec<Release>,
}

#[derive(Debug, Clone, PartialEq)]
enum Parser {
    /// Waiting for a level 2 header
    ///
    /// Idle + Start(Heading(2)) => ReadingHeader
    /// Idle + _ => Idle
    Idle,
    /// Reading the contents of a heading tag (`"## [1.2.3] - 2020-01-02"`)
    ///
    /// ReadingHeader + End(Heading(2...=> ReadingRelease | VersionParseError
    /// ReadingHeader + other => ReadingHeader (other added to buffer)
    ReadingHeader { buffer: Vec<(Event<'static>, Span)> },
    /// ReadingRelease + Start(Heading(2)) => ReadingHeader
    /// ReadingRelease + Start(Heading(3)) => ...
    /// ReadingRelease + other => ReadingRelease (ignored)
    ReadingRelease { release: Release },
}

impl Parser {
    /// Finalises any pending operations (e.g. imagine you are midway through
    /// parsing a section)
    fn flush<F>(self, state: &mut State, on_diagnostic: &mut F)
    where
        F: FnMut(Diagnostic<FileId>),
    {
        match self {
            Parser::Idle => {},
            Parser::ReadingHeader { buffer } => {
                match parse_header(buffer, state.file_id) {
                    Ok(release) => state.releases.push(release),
                    Err(diag) => on_diagnostic(diag),
                }
            },
            Parser::ReadingRelease { release } => state.releases.push(release),
        }
    }

    fn process<F>(
        self,
        event: Event<'static>,
        span: Span,
        state: &mut State,
        on_diagnostic: &mut F,
    ) -> Parser
    where
        F: FnMut(Diagnostic<FileId>),
    {
        match self {
            Parser::Idle => process_idle(event, span),
            Parser::ReadingHeader { buffer } => process_reading_header(
                event,
                span,
                buffer,
                state,
                on_diagnostic,
            ),
            Parser::ReadingRelease { release } => unimplemented!(),
        }
    }
}

impl Default for Parser {
    fn default() -> Parser { Parser::Idle }
}

fn process_idle(event: Event<'static>, span: Span) -> Parser {
    if event == Event::Start(Tag::Heading(2)) {
        Parser::ReadingHeader {
            buffer: vec![(event, span)],
        }
    } else {
        Parser::Idle
    }
}

fn process_reading_header<F>(
    event: Event<'static>,
    span: Span,
    mut buffer: Vec<(Event<'static>, Span)>,
    state: &mut State,
    on_diagnostic: &mut F,
) -> Parser
where
    F: FnMut(Diagnostic<FileId>),
{
    if event == Event::End(Tag::Heading(2)) {
        match parse_header(buffer, state.file_id) {
            Ok(empty_release) => {
                return Parser::ReadingRelease {
                    release: empty_release,
                }
            },
            Err(diag) => {
                // we couldn't parse the header, emit a diagnostic and skip to,
                // the next header
                on_diagnostic(diag);
                return Parser::Idle;
            },
        }
    }

    buffer.push((event, span));
    Parser::ReadingHeader { buffer }
}

fn parse_header(
    _buffer: Vec<(Event<'static>, Span)>,
    _file_id: FileId,
) -> Result<Release, Diagnostic<FileId>> {
    unimplemented!()
}

fn owned_event(ev: Event<'_>) -> Event<'static> {
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

fn owned_cow_str(s: CowStr<'_>) -> CowStr<'static> {
    match s {
        CowStr::Borrowed(_) => CowStr::from(s.into_string()),
        CowStr::Boxed(boxed) => CowStr::Boxed(boxed),
        CowStr::Inlined(inlined) => CowStr::Inlined(inlined),
    }
}

fn owned_tag(tag: Tag<'_>) -> Tag<'static> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use codespan::Files;

    fn dummy_file_id() -> FileId {
        let mut files = Files::new();
        files.add("asd", "fgh")
    }

    #[test]
    fn do_nothing_until_level_2_header() {
        let inputs = vec![Event::SoftBreak, Event::Text("asdf".into())];

        for event in inputs {
            let mut state = State {
                file_id: dummy_file_id(),
                releases: Vec::new(),
            };

            let new_parse_state = Parser::default().process(
                event,
                Span::initial(),
                &mut state,
                &mut |d| panic!("{:?}", d),
            );

            assert_eq!(new_parse_state, Parser::default());
        }
    }

    #[test]
    fn idle_plus_lvl2_header_starts_reading_header() {
        let event = Event::Start(Tag::Heading(2));
        let span = Span::initial();
        let should_be = Parser::ReadingHeader {
            buffer: vec![(event.clone(), span)],
        };

        let got = process_idle(event, span);

        assert_eq!(got, should_be);
    }
}
