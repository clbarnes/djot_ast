use djot_ast::AstNode;
use rstest::{fixture, rstest};

#[fixture]
fn readme_dj() -> &'static str {
    include_str!("fixtures/readme.dj")
}

#[fixture]
fn readme_json() -> &'static str {
    include_str!("fixtures/readme.dj.json")
}

#[rstest]
fn deser_readme(readme_json: &str) {
    let _node: AstNode = serde_json::from_str(readme_json).unwrap();
}
