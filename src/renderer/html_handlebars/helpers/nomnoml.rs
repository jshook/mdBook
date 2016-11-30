use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
//use htmlescape::*;

pub fn render_nomnoml(s: &str, path: &Path) -> String {
    // When replacing one thing in a string by something with a different length, the indices
    // after that will not correspond, we therefore have to store the difference to correct this
    let mut previous_end_index = 0;
    let mut replaced = String::new();

    for nomnoml in find_nomnomls(s, path) {

        if nomnoml.escaped {
            replaced.push_str(&s[previous_end_index..nomnoml.start_index - 1]);
            replaced.push_str(&s[nomnoml.start_index..nomnoml.end_index]);
            previous_end_index = nomnoml.end_index;
            continue;
        }

        // Check if the file exists
        if !nomnoml.nomnoml_file.exists() || !nomnoml.nomnoml_file.is_file() {
            warn!("[-] No file exists for {{{{#nomnoml }}}}\n    {}", nomnoml.nomnoml_file.to_str().unwrap());
            continue;
        }

        // Open file & read file
        let mut file = if let Ok(f) = File::open(&nomnoml.nomnoml_file) {
            f
        } else {
            continue;
        };
        let mut file_content = String::new();
        if let Err(_) = file.read_to_string(&mut file_content) {
            continue;
        };

        let replacement = String::new() + "<script id=\"nomnoml-text1\" type=\"text/plain\">\n" + &file_content + "\n</script>\n"
        + "<div id=\"nomnoml-view1\"></div>\n";
        replaced.push_str(&s[previous_end_index..nomnoml.start_index]);
        replaced.push_str(&replacement);
        previous_end_index = nomnoml.end_index;
        // println!("Nomnoml{{ {}, {}, {:?}, {} }}", nomnoml.start_index, nomnoml.end_index, nomnoml.nomnoml_file,
        // nomnoml.editable);
    }

    replaced.push_str(&s[previous_end_index..]);

    replaced
}

#[derive(PartialOrd, PartialEq, Debug)]
struct Nomnoml {
    start_index: usize,
    end_index: usize,
    nomnoml_file: PathBuf,
    editable: bool,
    escaped: bool,
}

fn find_nomnomls(s: &str, base_path: &Path) -> Vec<Nomnoml> {
    let mut nomnomls = vec![];
    for (i, _) in s.match_indices("{{#nomnoml") {
        debug!("[*]: find_nomnoml");

        let mut escaped = false;

        if i > 0 {
            if let Some(c) = s[i - 1..].chars().nth(0) {
                if c == '\\' {
                    escaped = true
                }
            }
        }
        // DON'T forget the "+ i" else you have an index out of bounds error !!
        let end_i = if let Some(n) = s[i..].find("}}") {
            n
        } else {
            continue;
        } + i + 2;

        debug!("s[{}..{}] = {}", i, end_i, s[i..end_i].to_string());

        // If there is nothing between "{{#nomnoml" and "}}" skip
        if end_i - 2 - (i + 10) < 1 {
            continue;
        }
        if s[i + 10..end_i - 2].trim().len() == 0 {
            continue;
        }

        debug!("{}", s[i + 10..end_i - 2].to_string());

        // Split on whitespaces
        let params: Vec<&str> = s[i + 10..end_i - 2].split_whitespace().collect();
        let mut editable = false;

        if params.len() > 1 {
            editable = if let Some(_) = params[1].find("editable") {
                true
            } else {
                false
            };
        }

        nomnomls.push(Nomnoml {
            start_index: i,
            end_index: end_i,
            nomnoml_file: base_path.join(PathBuf::from(params[0])),
            editable: editable,
            escaped: escaped,
        })
    }

    nomnomls
}




// ---------------------------------------------------------------------------------
//      Tests
//

#[test]
fn test_find_nomnomls_no_nomnoml() {
    let s = "Some random text without nomnoml...";
    assert!(find_nomnomls(s, Path::new("")) == vec![]);
}

#[test]
fn test_find_nomnomls_partial_nomnoml() {
    let s = "Some random text with {{#nomnoml...";
    assert!(find_nomnomls(s, Path::new("")) == vec![]);
}

#[test]
fn test_find_nomnomls_empty_nomnoml() {
    let s = "Some random text with {{#nomnoml}} and {{#nomnoml   }}...";
    assert!(find_nomnomls(s, Path::new("")) == vec![]);
}

#[test]
fn test_find_nomnomls_simple_nomnoml() {
    let s = "Some random text with {{#nomnoml file.rs}} and {{#nomnoml test.rs }}...";

    println!("\nOUTPUT: {:?}\n", find_nomnomls(s, Path::new("")));

    assert!(find_nomnomls(s, Path::new("")) ==
            vec![Nomnoml {
                     start_index: 22,
                     end_index: 42,
                     nomnoml_file: PathBuf::from("file.rs"),
                     editable: false,
                     escaped: false,
                 },
                 Nomnoml {
                     start_index: 47,
                     end_index: 68,
                     nomnoml_file: PathBuf::from("test.rs"),
                     editable: false,
                     escaped: false,
                 }]);
}

#[test]
fn test_find_nomnomls_complex_nomnoml() {
    let s = "Some random text with {{#nomnoml file.rs editable}} and {{#nomnoml test.rs editable }}...";

    println!("\nOUTPUT: {:?}\n", find_nomnomls(s, Path::new("dir")));

    assert!(find_nomnomls(s, Path::new("dir")) ==
            vec![Nomnoml {
                     start_index: 22,
                     end_index: 51,
                     nomnoml_file: PathBuf::from("dir/file.rs"),
                     editable: true,
                     escaped: false,
                 },
                 Nomnoml {
                     start_index: 56,
                     end_index: 86,
                     nomnoml_file: PathBuf::from("dir/test.rs"),
                     editable: true,
                     escaped: false,
                 }]);
}

#[test]
fn test_find_nomnomls_escaped_nomnoml() {
    let s = "Some random text with escaped nomnoml \\{{#nomnoml file.rs editable}} ...";

    println!("\nOUTPUT: {:?}\n", find_nomnomls(s, Path::new("")));

    assert!(find_nomnomls(s, Path::new("")) ==
            vec![
        Nomnoml{start_index: 39, end_index: 68, nomnoml_file: PathBuf::from("file.rs"), editable: true, escaped: true},
    ]);
}
