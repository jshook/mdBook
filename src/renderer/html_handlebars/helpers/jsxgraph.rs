extern crate regex;

lazy_static! {
        static ref BLOCK_RE: regex::Regex = regex::Regex::new(r"(?ms)\{\{#jsxgraph\s+(?P<spec>.*?)\}\}").unwrap();
    }

pub fn render_jsxgraph(s: &str) -> String {
    BLOCK_RE.replace_all(s, "\n<script type=\"text/javascript\">\n$spec\n</script>\n")
}

