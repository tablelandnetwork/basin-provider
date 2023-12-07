use curve25519_dalek::RistrettoPoint;
use sha2::Sha512;
use std::fmt::Debug;

/// Based on `EllipticCurveMultisetHash` from fastcrypto.
#[derive(Default, Clone)]
pub struct RistrettoMultisetHash {
    accumulator: RistrettoPoint,
}

impl PartialEq for RistrettoMultisetHash {
    fn eq(&self, other: &Self) -> bool {
        self.accumulator == other.accumulator
    }
}

impl RistrettoMultisetHash {
    pub fn insert<Data: AsRef<[u8]>>(&mut self, item: Data) {
        self.accumulator += Self::hash_to_point(item);
    }

    pub fn hash(&self) -> [u8; 32] {
        self.accumulator.compress().to_bytes()
    }
}

impl RistrettoMultisetHash {
    /// Hash the given item into a RistrettoPoint to be used by the insert and remove methods.
    fn hash_to_point<Data: AsRef<[u8]>>(item: Data) -> RistrettoPoint {
        RistrettoPoint::hash_from_bytes::<Sha512>(item.as_ref())
    }
}

impl Debug for RistrettoMultisetHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Accumulator").finish()
    }
}

pub struct Hasher {
    hashset: RistrettoMultisetHash,
}

impl Default for Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher {
    pub fn new() -> Self {
        Self {
            hashset: RistrettoMultisetHash::default(),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.hashset.insert(data);
    }

    pub fn finalize(&mut self, output: &mut [u8; 32]) {
        let hash = self.hashset.hash();
        output.copy_from_slice(&hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_insert_and_hash() {
        let mut hash = RistrettoMultisetHash::default();
        let item = "test_data";
        hash.insert(item);

        assert_ne!(hash.accumulator, RistrettoPoint::default());
        assert_eq!(
            hex::encode(hash.hash()),
            "a856da6eb4d2abb55944eb2005cda29b358919e1a9c13208c0ab342c05fd654b"
        );
    }

    #[test]
    fn test_inserts_equal() {
        let mut hash1 = RistrettoMultisetHash::default();
        let mut hash2 = RistrettoMultisetHash::default();

        let item = "test_data";
        hash1.insert(item);
        hash2.insert(item);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_inserts_not_equal() {
        let mut hash1 = RistrettoMultisetHash::default();
        let mut hash2 = RistrettoMultisetHash::default();

        hash1.insert("item1");
        hash2.insert("item2");

        assert_ne!(hash1, hash2);
    }
}
