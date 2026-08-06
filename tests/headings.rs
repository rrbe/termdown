mod common;

use std::path::Path;

use common::run_termdown;

const BASIC_FIXTURE: &str = "fixtures/specialized/headings-basic.md";

#[test]
fn piped_output_preserves_heading_text_without_images() {
    let stdout = run_termdown(Path::new(BASIC_FIXTURE));
    let text = String::from_utf8(stdout).expect("cat output should be UTF-8");

    assert!(
        !text.contains("\x1b_G"),
        "piped output must not contain images"
    );
    assert!(text.contains("Heading One"));
    assert!(text.contains("Heading Two"));
    assert!(text.contains("Heading Three"));
}
