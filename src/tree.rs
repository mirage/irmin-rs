use crate::Type;

pub enum Tree<Hash: Type> {
    Hash(Hash),
    Id(isize),
    Local(Vec<u8>),
}

/*impl<Hash: Type> Type for Tree<Hash> {
    fn encode_bin<W: std::io::Write>(&self, w: W) -> std::io::Result<usize> {}

    fn decode_bin<R: std::io::Read>(r: R) -> std::io::Result<Self> {}
}*/
