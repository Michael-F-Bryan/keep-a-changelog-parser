use codespan::{Files, Span};

macro_rules! sanity_check {
    ($filename:ident) => {
        #[test]
        #[ignore]
        fn $filename() {
            let src = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/data/",
                stringify!($filename),
                ".md"
            ));

            let mut diagnostics = Vec::new();
            let mut files = Files::new();
            let file_id = files.add(stringify!($filename), src);

            let _ = keep_a_changelog_parser::parse(file_id, src, |diag| {
                diagnostics.push(diag)
            });

            assert!(diagnostics.is_empty());
        }
    };
}

sanity_check!(keep_a_changelog);

#[test]
#[ignore]
fn crate_changelog() {
    let src =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/CHANGELOG.md"));

    let mut diagnostics = Vec::new();
    let mut files = Files::new();
    let file_id = files.add(stringify!($filename), src);

    let got = keep_a_changelog_parser::parse(file_id, src, |diag| {
        diagnostics.push(diag)
    });

    assert_eq!(got.span, Span::from_str(src));
    assert_eq!(got.releases.len(), 1);
    let unreleased = &got.releases[0];
    assert!(unreleased.date.is_none());
    assert!(unreleased.version.is_none());
}
