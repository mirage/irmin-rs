use crate::{Fixed, Type};

pub use blake2::{Blake2b, Digest};
pub use sha1::Sha1;

#[derive(Debug, Clone, PartialEq)]
pub struct Hash<T: Fixed>(pub(crate) Vec<u8>, pub(crate) std::marker::PhantomData<T>);

impl Fixed for Blake2b {
    const SIZE: usize = 64;

    fn hash(s: impl AsRef<[u8]>) -> Hash<Blake2b> {
        let digest = blake2::Blake2b::digest(s.as_ref());
        Hash(digest.as_slice().to_vec(), std::marker::PhantomData)
    }
}

impl Fixed for Sha1 {
    const SIZE: usize = 20;

    fn hash(s: impl AsRef<[u8]>) -> Hash<Sha1> {
        let mut hash = sha1::Sha1::default();
        hash.update(s.as_ref());
        let digest = hash.digest();
        Hash(digest.bytes().to_vec(), std::marker::PhantomData)
    }
}

impl<T: Fixed> Hash<T> {
    pub fn hash(s: impl AsRef<[u8]>) -> Hash<T> {
        T::hash(s)
    }

    pub fn new(s: impl Into<Vec<u8>>) -> Option<Hash<T>> {
        let s = s.into();
        if s.len() != T::SIZE {
            return None;
        }

        Some(Hash(s.into(), std::marker::PhantomData))
    }
}

impl<T: Fixed> Type for Hash<T> {
    fn name() -> String {
        String::new()
    }

    fn encode_bin<W: std::io::Write>(&self, mut w: W) -> std::io::Result<usize> {
        w.write_all(&self.0)?;
        Ok(T::SIZE)
    }

    fn decode_bin<R: std::io::Read>(mut r: R) -> std::io::Result<Self> {
        let mut data = vec![0u8; T::SIZE];
        r.read_exact(data.as_mut_slice())?;
        Ok(Hash::new(data).unwrap())
    }
}
