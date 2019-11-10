use std::{env, fs};
use std::path::PathBuf;

use ::serde_yaml;

fn main() {

    // Get the schema and output directory.
    let schema_dir = fs::canonicalize(&PathBuf::from("../schema")).unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    assert!(schema_dir.is_dir());
    assert!(out_dir.is_dir());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", schema_dir.to_string_lossy());

    let files = vec!["choose_date.yaml"];
    for filename in files {

        // Create path and make sure it causes cache invalidation on change.
        let yaml_pth = schema_dir.join(filename);
        println!("cargo:rerun-if-changed={}", yaml_pth.to_string_lossy());

        // Read and parse the yaml.
        let yaml_str = fs::read_to_string(&yaml_pth).unwrap();
        let yaml_code: serde_json::Value = serde_yaml::from_str(&yaml_str).unwrap();

        // Write to json.
        let json_pth = {
            let mut pth = PathBuf::from(yaml_pth.file_name().unwrap());
            pth = out_dir.join(pth);
            pth.set_extension("json");
            pth
        };
        let json_str = serde_json::to_string(&yaml_code).unwrap();
        fs::write(json_pth, json_str).unwrap();
    }

    //TODO @mark: json -> objects
}
