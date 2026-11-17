use pulldown_cmark::{html, Options, Parser};
use std::{env, fs, path::Path};

/// `src/data/content/*.md` と `src/data/content/*/explanation.md` を `$OUT_DIR/works/*.html` に変換する。
fn main() {
    let src_dir = Path::new("src/data/content");
    let out_dir = Path::new(&env::var("OUT_DIR").unwrap()).join("works");
    fs::create_dir_all(&out_dir).unwrap();
    println!("cargo:rerun-if-changed=src/data/content");

    for entry in fs::read_dir(src_dir).unwrap() {
        let mut path = entry.unwrap().path();
        // `<name>/explanation.md` は `<name>.html` として出力する。
        let dir_name = path.is_dir().then(|| path.file_name().unwrap().to_owned());
        if dir_name.is_some() {
            path = path.join("explanation.md");
        }
        if path.extension().is_some_and(|ext| ext == "md") {
            let markdown = fs::read_to_string(&path).unwrap();
            let mut rendered = String::new();
            html::push_html(
                &mut rendered,
                Parser::new_ext(&markdown, Options::ENABLE_TABLES),
            );
            let name = dir_name
                .as_deref()
                .unwrap_or_else(|| path.file_stem().unwrap());
            fs::write(out_dir.join(name).with_extension("html"), rendered).unwrap();
        }
    }
}
