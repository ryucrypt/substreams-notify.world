#[allow(dead_code)]
pub const ACCOUNT: Option<&'static str> = Some("notify.world");
pub mod types {
    use substreams_antelope::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct MiningData2 {
        pub invalid: Uint8,
        pub error: String,
        pub delay: Uint16,
        pub difficulty: Uint8,
        pub ease: Uint16,
        pub luck: Uint16,
        pub commission: Uint16,
        pub eases: Vec<PairStringUint16>,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PairStringAsset {
        pub key: String,
        pub value: Asset,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PairStringUint16 {
        pub first: String,
        pub second: Uint16,
    }
}
pub mod actions {
    use substreams_antelope::types::*;
    use substreams_antelope::decoder::decode;
    #[allow(unused_imports)]
    use super::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Logmine {
        pub miner: Name,
        pub params: MiningData2,
        pub bounty: Asset,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub land_id: Uint64,
        pub planet_name: Name,
        pub landowner: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::vec_str_or_u64")]
        pub bag_items: Vec<Uint64>,
        pub offset: Uint32,
        pub landowner_share: Asset,
        pub pool_amounts: Vec<PairStringAsset>,
    }
    impl substreams_antelope::Action for Logmine {
        const NAME: &'static str = "logmine";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
}