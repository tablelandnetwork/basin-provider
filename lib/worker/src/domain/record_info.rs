use multibase::Base;
use serde::Serialize;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Serialize)]
pub struct RecordInfo {
    pub cid: String,
    pub timestamp: i64,
    pub cache_expiry: Option<NaiveDateTime>,
}

impl RecordInfo {
    pub fn new(cid: Vec<u8>, timestamp: i64, cache_expiry: Option<NaiveDateTime>) -> RecordInfo {
        let cid_str = multibase::encode::<Vec<u8>>(Base::Base32Lower, cid);

        RecordInfo {
            cid: cid_str,
            timestamp,
            cache_expiry,
        }
    }
}
