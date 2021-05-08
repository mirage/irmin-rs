use crate::{Hash, Type};

pub use blake2::{Blake2b, Digest};
pub use sha1::Sha1;

#[derive(Debug, Clone)]
pub struct HashRef<T: Hash>(pub(crate) Vec<u8>, pub(crate) std::marker::PhantomData<T>);

impl<T: Hash> PartialEq for HashRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Hash for Blake2b {
    fn size() -> usize {
        64
    }

    fn hash(s: impl AsRef<[u8]>) -> HashRef<Blake2b> {
        let digest = blake2::Blake2b::digest(s.as_ref());
        HashRef(digest.as_slice().to_vec(), std::marker::PhantomData)
    }
}

impl Hash for Sha1 {
    fn size() -> usize {
        20
    }

    fn hash(s: impl AsRef<[u8]>) -> HashRef<Sha1> {
        let mut hash = sha1::Sha1::default();
        hash.update(s.as_ref());
        let digest = hash.digest();
        HashRef(digest.bytes().to_vec(), std::marker::PhantomData)
    }
}

impl<T: Hash> HashRef<T> {
    pub fn hash(s: impl AsRef<[u8]>) -> HashRef<T> {
        T::hash(s)
    }

    pub fn new(s: impl Into<Vec<u8>>) -> Option<HashRef<T>> {
        let s = s.into();
        if s.len() != T::size() {
            return None;
        }

        Some(HashRef(s.into(), std::marker::PhantomData))
    }
}

impl<T: Hash> Type for HashRef<T> {
    fn encode_bin<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<usize> {
        w.write_all(&self.0)?;
        Ok(T::size())
    }

    fn decode_bin<R: std::io::Read>(r: &mut R) -> std::io::Result<Self> {
        let mut data = vec![0u8; T::size()];
        r.read_exact(data.as_mut_slice())?;
        Ok(HashRef::new(data).unwrap())
    }
}
