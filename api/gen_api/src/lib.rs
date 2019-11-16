#![feature(rustc_private)]

//TODO @mark: Add RPC: https://jsonrpc.org/historical/json-schema-service-descriptor.html

use ::rmp_serde;

include!(concat!(env!("OUT_DIR"), "/schema.rs"));

trait MsgPack : Sized {
    fn as_msg_pack(&mut self) -> Result<Vec<u8>, rmp_serde::encode::Error>;
    fn from_msg_pack(data: &[u8]) -> Result<Self, rmp_serde::decode::Error>;
}

impl <'a, T> MsgPack for T where T: Serialize, for<'de> T: serde::Deserialize<'de> {
    fn as_msg_pack(&mut self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        let mut buf = Vec::new();
        self.serialize(&mut rmp_serde::Serializer::new(&mut buf))?;
        Ok(buf)
    }

    fn from_msg_pack(data: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_read(data)
    }
}

#[cfg(test)]
mod tests {
    use super::Api;
    use super::MsgPack;

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
        let _msg_pack = api.as_msg_pack().unwrap();
    }
}
