extern crate regex;

//use std::path::{Path};
//use std::fs::File;
//use std::io::Read;

lazy_static! {
        static ref BLOCK_RE: regex::Regex = regex::Regex::new(r"(?ms)\{\{#nomnoml\s+(?P<spec>.*?)\}\}").unwrap();
//        static ref BLOCK_RE: regex::Regex = regex::Regex::new(r"(?ms)(?P<sigil>\{\{#)\s*(?P<helper>nomnoml)\s+(P?<spec>.+?)\}\}").unwrap();
    }

pub fn render_nomnoml(s: &str) -> String {

    BLOCK_RE.replace_all(s, "\n<script class=\"nomnoml-text\" type=\"text/plain\">\n$spec\n</script>\n")

}

// ---------------------------------------------------------------------------------
//      Tests
//

#[test]
fn test_oneline_replacement() {
    let s = "Some random text with {{#nomnoml part1}}...";
    let result :String = render_nomnoml(s);
    println!("result:{}", result);
    assert!(result.as_str() == "Some random text with \n<script class=\"nomnoml-text\" type=\"text/plain\">\npart1\n</script>\n...");
}

#[test]
fn test_twoline_replacement() {
    let s = "Some random text with {{#nomnoml part1\npart2\n}}...";
    let result :String = render_nomnoml(s);
    println!("result:{}", result);
    assert!(result.as_str() == "Some random text with \n<script class=\"nomnoml-text\" type=\"text/plain\">\npart1\npart2\n\n</script>\n...");
}
