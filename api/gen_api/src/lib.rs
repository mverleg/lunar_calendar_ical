#![feature(rustc_private)]

include!(concat!(env!("OUT_DIR"), "/schema.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn generated_code_exists() {
        use super::Api;
        use super::Date;
        let api: Api = serde_json::from_str(r#"{
            "initial_date": {
                "calendar": "LunarChina",
                "format": "yyyy-MM-dd",
                "value": "2019-11-11"
            },
            "name": "你好",
            "repeat_mode": "yearly",
            "include_repetition_count": false
        }"#).unwrap();
    }
}
