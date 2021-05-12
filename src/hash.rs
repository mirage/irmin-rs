use crate::Type;

use blake2::Digest;

macro_rules! hash_type {
    ($x: ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $x(pub Vec<u8>);
        impl AsRef<[u8]> for $x {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }

        impl Type for $x {
            fn encode_bin<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<usize> {
                w.write_all(&self.0)?;
                Ok(Self::size())
            }

            fn decode_bin<R: std::io::Read>(r: &mut R) -> std::io::Result<Self> {
                let mut data = vec![0u8; Self::size()];
                r.read_exact(data.as_mut_slice())?;
                Ok($x(data))
            }
        }
    };
}

hash_type!(Blake2b);
hash_type!(Sha1);

pub trait Hash: Type + Clone + Sized + PartialEq {
    fn size() -> usize;

    fn name() -> &'static str;

    fn hash(x: impl AsRef<[u8]>) -> Self;
}

impl Hash for Blake2b {
    fn size() -> usize {
        64
    }

    fn name() -> &'static str {
        "blake2b"
    }

    fn hash(s: impl AsRef<[u8]>) -> Self {
        let digest = blake2::Blake2b::digest(s.as_ref());
        Blake2b(digest.as_slice().to_vec())
    }
}

impl Hash for Sha1 {
    fn size() -> usize {
        20
    }

    fn name() -> &'static str {
        "sha1"
    }

    fn hash(s: impl AsRef<[u8]>) -> Self {
        let mut hash = sha1::Sha1::default();
        hash.update(s.as_ref());
        let digest = hash.digest();
        Sha1(digest.bytes().to_vec())
    }
}
