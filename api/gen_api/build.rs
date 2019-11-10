use std::{env, fs};
use std::path::PathBuf;

use ::serde_yaml;
use chrono::Local;

fn main() {

    // Get the schema and output directory.
    let schema_dir = fs::canonicalize(&PathBuf::from("../schema")).unwrap();
    let out_dir = pathdiff::diff_paths(
        &PathBuf::from(env::var("OUT_DIR").unwrap()),
        &env::current_dir().unwrap(),
    ).expect("cannot express output directory relative to cargo directory");
    let entity_pth = out_dir.join(PathBuf::from("schema.rs"));
    println!("out: {}", out_dir.to_string_lossy());  //TODO @mark: TEMPORARY! REMOVE THIS!
    assert!(schema_dir.is_dir());
    assert!(out_dir.is_dir());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", schema_dir.to_string_lossy());
    println!("cargo:rerun-if-changed={}", entity_pth.to_string_lossy());

    let mut json_pths = vec![];
    let yaml_files = vec!["choose_date.yaml"];  //TODO @mark:
    for filename in yaml_files {

        // Create path and make sure it causes cache invalidation on change.
        let yaml_pth = schema_dir.join(filename);
        println!("cargo:rerun-if-changed={}", yaml_pth.to_string_lossy());

        // Read and parse the yaml.
        let yaml_str = fs::read_to_string(&yaml_pth).unwrap();
        let yaml_code: serde_json::Value = serde_yaml::from_str(&yaml_str).unwrap();

        // Write to json.
        let filename = {
            let mut pth = PathBuf::from(yaml_pth.file_name().unwrap());
            pth.set_extension("json");
            pth
        };
        let json_pth = out_dir.join(filename);
        json_pths.push(json_pth.to_string_lossy().into_owned());
        let json_str = serde_json::to_string(&yaml_code).unwrap();
        fs::write(json_pth, json_str).unwrap();
    }

    // Generate the macro code.
    let mut entity_code = vec![
        format!("// automatically generated at {} from json schema yaml files", Local::now().format("%Y-%m-%d %H:%M:%S")),
        "extern crate serde;".to_owned(),
        "extern crate schemafy_core;".to_owned(),
        "extern crate serde_json;".to_owned(),
        "use serde::{Serialize, Deserialize};".to_owned(),
        "schemafy::schemafy!(".to_owned(),
        //"\troot::Api,".to_owned(),
    ];
    for filename in json_pths {
        entity_code.push(format!("\t\"{}\"", filename));
    }
    entity_code.push(");".to_owned());
    fs::write(entity_pth, entity_code.join("\n")).unwrap();

    //TODO @mark: json -> objects
}
