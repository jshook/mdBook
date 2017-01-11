extern crate regex;

lazy_static! {
        static ref BLOCK_RE: regex::Regex = regex::Regex::new(r"(?ms)\{\{#functionplot\s+(?P<spec>.*?)\}\}").unwrap();
    }

pub fn render_functionplot(s: &str) -> String {
    BLOCK_RE.replace_all(s, "\n<script>\nfunctionPlot($spec);\n</script>\n")
}

