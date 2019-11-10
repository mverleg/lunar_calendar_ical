extern crate schemafy_core;
extern crate serde;

use std::env;
use std::path::PathBuf;

//static OUT_DIR: PathBuf = {
//    let pth = PathBuf::from(env::var("OUT_DIR").unwrap());
//    assert!(pth.is_dir());
//    pth
//};
//
//schemafy::schemafy!(
//    OUT_DIR.to_string_lossy()
//);

//fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let nested: Defnested = serde_json::from_str(r#"{ "append": "abc" }"#)?;
//    assert_eq!(nested.append, Some("abc".to_string()));
//    Ok(())
//}
