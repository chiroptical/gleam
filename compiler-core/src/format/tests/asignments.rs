use crate::assert_format;

// https://github.com/gleam-lang/gleam/issues/2095
#[test]
fn comment() {
    assert_format!(
        r#"pub fn main() {
  // Hello
  let x = 1
  x
}
"#
    );
}
