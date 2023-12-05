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

impl Eq for RistrettoMultisetHash {}

impl RistrettoMultisetHash {
    pub fn insert<Data: AsRef<[u8]>>(&mut self, item: Data) {
        self.accumulator += Self::hash_to_point(item);
    }

    pub fn insert_all<It, Data>(&mut self, items: It)
    where
        It: IntoIterator<Item = Data>,
        Data: AsRef<[u8]>,
    {
        for i in items {
            self.insert(i);
        }
    }

    pub fn union(&mut self, other: &Self) {
        self.accumulator += other.accumulator;
    }

    pub fn difference(&mut self, other: &Self) {
        self.accumulator -= other.accumulator;
    }

    pub fn remove<Data: AsRef<[u8]>>(&mut self, item: Data) {
        self.accumulator -= Self::hash_to_point(item);
    }

    pub fn remove_all<It, Data>(&mut self, items: It)
    where
        It: IntoIterator<Item = Data>,
        Data: AsRef<[u8]>,
    {
        for i in items {
            self.remove(i);
        }
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
