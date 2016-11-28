use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;


pub fn render_mermaid(s: &str, path: &Path) -> String {
    // When replacing one thing in a string by something with a different length, the indices
    // after that will not correspond, we therefore have to store the difference to correct this
    let mut previous_end_index = 0;
    let mut replaced = String::new();

    for mermaid in find_mermaids(s, path) {

        if mermaid.escaped {
            replaced.push_str(&s[previous_end_index..mermaid.start_index - 1]);
            replaced.push_str(&s[mermaid.start_index..mermaid.end_index]);
            previous_end_index = mermaid.end_index;
            continue;
        }

        // Check if the file exists
        if !mermaid.rust_file.exists() || !mermaid.rust_file.is_file() {
            warn!("[-] No file exists for {{{{#mermaid }}}}\n    {}", mermaid.rust_file.to_str().unwrap());
            continue;
        }

        // Open file & read file
        let mut file = if let Ok(f) = File::open(&mermaid.rust_file) {
            f
        } else {
            continue;
        };
        let mut file_content = String::new();
        if let Err(_) = file.read_to_string(&mut file_content) {
            continue;
        };

        let replacement = String::new() + "<div class=\"mermaid\">\n" + &file_content + "\n</div>";

        replaced.push_str(&s[previous_end_index..mermaid.start_index]);
        replaced.push_str(&replacement);
        previous_end_index = mermaid.end_index;
        // println!("Mermaid{{ {}, {}, {:?}, {} }}", mermaid.start_index, mermaid.end_index, mermaid.rust_file,
        // mermaid.editable);
    }

    replaced.push_str(&s[previous_end_index..]);

    replaced
}

#[derive(PartialOrd, PartialEq, Debug)]
struct Mermaid {
    start_index: usize,
    end_index: usize,
    rust_file: PathBuf,
    editable: bool,
    escaped: bool,
}

fn find_mermaids(s: &str, base_path: &Path) -> Vec<Mermaid> {
    let mut mermaids = vec![];
    for (i, _) in s.match_indices("{{#mermaid") {
        debug!("[*]: find_mermaid");

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

        // If there is nothing between "{{#mermaid" and "}}" skip
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

        mermaids.push(Mermaid {
            start_index: i,
            end_index: end_i,
            rust_file: base_path.join(PathBuf::from(params[0])),
            editable: editable,
            escaped: escaped,
        })
    }

    mermaids
}




// ---------------------------------------------------------------------------------
//      Tests
//

#[test]
fn test_find_mermaids_no_mermaid() {
    let s = "Some random text without mermaid...";
    assert!(find_mermaids(s, Path::new("")) == vec![]);
}

#[test]
fn test_find_mermaids_partial_mermaid() {
    let s = "Some random text with {{#mermaid...";
    assert!(find_mermaids(s, Path::new("")) == vec![]);
}

#[test]
fn test_find_mermaids_empty_mermaid() {
    let s = "Some random text with {{#mermaid}} and {{#mermaid   }}...";
    assert!(find_mermaids(s, Path::new("")) == vec![]);
}

#[test]
fn test_find_mermaids_simple_mermaid() {
    let s = "Some random text with {{#mermaid file.rs}} and {{#mermaid test.rs }}...";

    println!("\nOUTPUT: {:?}\n", find_mermaids(s, Path::new("")));

    assert!(find_mermaids(s, Path::new("")) ==
            vec![Mermaid {
                     start_index: 22,
                     end_index: 42,
                     rust_file: PathBuf::from("file.rs"),
                     editable: false,
                     escaped: false,
                 },
                 Mermaid {
                     start_index: 47,
                     end_index: 68,
                     rust_file: PathBuf::from("test.rs"),
                     editable: false,
                     escaped: false,
                 }]);
}

#[test]
fn test_find_mermaids_complex_mermaid() {
    let s = "Some random text with {{#mermaid file.rs editable}} and {{#mermaid test.rs editable }}...";

    println!("\nOUTPUT: {:?}\n", find_mermaids(s, Path::new("dir")));

    assert!(find_mermaids(s, Path::new("dir")) ==
            vec![Mermaid {
                     start_index: 22,
                     end_index: 51,
                     rust_file: PathBuf::from("dir/file.rs"),
                     editable: true,
                     escaped: false,
                 },
                 Mermaid {
                     start_index: 56,
                     end_index: 86,
                     rust_file: PathBuf::from("dir/test.rs"),
                     editable: true,
                     escaped: false,
                 }]);
}

#[test]
fn test_find_mermaids_escaped_mermaid() {
    let s = "Some random text with escaped mermaid \\{{#mermaid file.rs editable}} ...";

    println!("\nOUTPUT: {:?}\n", find_mermaids(s, Path::new("")));

    assert!(find_mermaids(s, Path::new("")) ==
            vec![
        Mermaid{start_index: 39, end_index: 68, rust_file: PathBuf::from("file.rs"), editable: true, escaped: true},
    ]);
}
