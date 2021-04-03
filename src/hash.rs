use crate::{Fixed, Type};

#[derive(Debug, Clone, PartialEq)]
pub struct Hash<T: Fixed>(pub(crate) String, pub(crate) std::marker::PhantomData<T>);

pub struct Blake2b;
pub struct Sha1;

impl Fixed for Blake2b {
    const SIZE: usize = 64;
}

impl Fixed for Sha1 {
    const SIZE: usize = 20;
}

impl<T: Fixed> Hash<T> {
    pub(crate) fn from_string(s: impl Into<String>) -> Hash<T> {
        Hash(s.into(), std::marker::PhantomData)
    }

    pub fn new(s: impl Into<String>) -> Option<Hash<T>> {
        let s = s.into();
        if s.len() != T::SIZE {
            return None;
        }

        Some(Hash::from_string(s))
    }
}

impl<T: Fixed> Type for Hash<T> {
    fn encode_bin<W: std::io::Write>(&self, mut w: W) -> std::io::Result<usize> {
        w.write_all(self.0.as_bytes())?;
        Ok(T::SIZE)
    }

    fn decode_bin<R: std::io::Read>(mut r: R) -> std::io::Result<Self> {
        let mut data = vec![0u8; T::SIZE];
        r.read_exact(data.as_mut_slice())?;
        let s = String::from_utf8_lossy(&data).to_string();
        Ok(Hash::from_string(s))
    }
}
