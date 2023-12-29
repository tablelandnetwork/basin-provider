use multibase::decode;

#[derive(Debug, Clone)]
pub struct Cid(Vec<u8>);

impl Cid {
    pub fn from(cid: String) -> Result<Cid, String> {
        let cid = match decode(cid) {
            Ok((_, v)) => v,
            Err(_) => return Err("failed to decode cid".to_string()),
        };

        Ok(Cid(cid))
    }
}

impl AsRef<Vec<u8>> for Cid {
    fn as_ref(&self) -> &Vec<u8> {
        &self.0
    }
}
