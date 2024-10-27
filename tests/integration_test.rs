use djot_ast::AstNode;
use rstest::{fixture, rstest};

#[fixture]
fn pandoc_json() -> &'static str {
    include_str!("fixtures/pandoc-manual.json")
}

#[fixture]
fn readme_dj() -> &'static str {
    include_str!("fixtures/readme.dj")
}

#[fixture]
fn readme_json() -> &'static str {
    include_str!("fixtures/readme.dj.json")
}

fn can_deser(s: &str) -> AstNode {
    serde_json::from_str(s).unwrap()
}

#[rstest]
fn deser_readme(readme_json: &str) {
    can_deser(readme_json);
}

#[rstest]
fn deser_pandoc(pandoc_json: &str) {
    can_deser(pandoc_json);
}
