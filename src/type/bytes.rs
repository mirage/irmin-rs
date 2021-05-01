#[derive(Debug, PartialEq, PartialOrd)]
pub enum Bytes<'a> {
    Owned(Vec<u8>),
    Ref(&'a [u8]),
}

impl<'a> AsRef<[u8]> for Bytes<'a> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Bytes::Owned(x) => x.as_ref(),
            Bytes::Ref(x) => x,
        }
    }
}

impl<'a> From<&'a [u8]> for Bytes<'a> {
    fn from(x: &'a [u8]) -> Self {
        Bytes::Ref(x)
    }
}

impl<'a> From<Vec<u8>> for Bytes<'a> {
    fn from(x: Vec<u8>) -> Self {
        Bytes::Owned(x)
    }
}
