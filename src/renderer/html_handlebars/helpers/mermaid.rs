extern crate regex;

lazy_static! {
        static ref BLOCK_RE: regex::Regex = regex::Regex::new(r"(?ms)\{\{#mermaid\s+(?P<spec>.*?)\}\}").unwrap();
    }

pub fn render_mermaid(s: &str) -> String {
    BLOCK_RE.replace_all(s, "\n<div class=\"mermaid\">\n$spec\n</div>\n")
}

// ---------------------------------------------------------------------------------
//      Tests
//

#[test]
fn test_oneline_replacement() {
    let s = "Some random text with {{#mermaid part1}}...";
    let result: String = render_nomnoml(s);
    println!("result:{}", result);
    assert!(result.as_str() == "Some random text with \n<div class=\"mermaid\">\npart1\n</div>\n...");
}

#[test]
fn test_twoline_replacement() {
    let s = "Some random text with {{#mermaid part1\npart2\n}}...";
    let result: String = render_nomnoml(s);
    println!("result:{}", result);
    assert!(result.as_str() == "Some random text with \n<div class=\"mermaid\">\npart1\npart2\n\n</div>\n...");
}
