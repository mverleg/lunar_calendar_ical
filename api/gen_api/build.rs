use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn main() {

    // Get the schema directory.
    let schema_dir = fs::canonicalize(&PathBuf::from("../schema")).unwrap();
    assert!(schema_dir.is_dir());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", schema_dir.to_string_lossy());

    let files = vec!["choose_date.yaml"];
    for filename in files {
        let file = schema_dir.join(filename);
        println!("cargo:rerun-if-changed={}", file.to_string_lossy());

        
    }

    //TODO @mark: read yaml
    //TODO @mark: write json
    //TODO @mark: json -> objects
}
