#![feature(rustc_private)]

use ::rmp_serde;

include!(concat!(env!("OUT_DIR"), "/schema.rs"));

trait ToMsgPack {
    fn as_msg_pack(&mut self) -> Result<Vec<u8>, rmp_serde::encode::Error>;
}

impl <T> ToMsgPack for T where T: Serialize {
    fn as_msg_pack(&mut self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        let mut buf = Vec::new();
        self.serialize(&mut rmp_serde::Serializer::new(&mut buf))?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::Api;
    use super::ToMsgPack;

    #[test]
    fn generated_code_exists() {
        let mut api: Api = serde_json::from_str(r#"{
            "initial_date": {
                "calendar": "LunarChina",
                "format": "yyyy-MM-dd",
                "value": "2019-11-11"
            },
            "name": "你好",
            "repeat_mode": "yearly",
            "include_repetition_count": false
        }"#).unwrap();
        let msg_pack = api.as_msg_pack().unwrap();
        println!("{:?}", msg_pack);  //TODO @mark: TEMPORARY! REMOVE THIS!
        println!("{:?}", msg_pack.len());  //TODO @mark: TEMPORARY! REMOVE THIS!
        panic!();  //TODO @mark: TEMPORARY! REMOVE THIS!
    }
}
