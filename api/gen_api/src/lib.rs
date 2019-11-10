extern crate serde;
extern crate schemafy_core;
extern crate serde_json;

use serde::{Serialize, Deserialize};

schemafy::schemafy!(
    "../schema/choose_date.yaml"
);

//fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let nested: Defnested = serde_json::from_str(r#"{ "append": "abc" }"#)?;
//    assert_eq!(nested.append, Some("abc".to_string()));
//    Ok(())
//}
