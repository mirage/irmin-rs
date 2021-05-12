use crate::{irmin, Hash, Info, Type};

#[derive(Debug, Clone, PartialEq, Type)]
pub struct Commit<H: Hash> {
    pub node: H,
    pub parents: Vec<H>,
    pub info: Info,
}

impl<H: Hash> Commit<H> {
    pub fn new(node: H, parents: impl AsRef<[H]>, info: Info) -> Commit<H> {
        Commit {
            node,
            parents: parents.as_ref().to_vec(),
            info,
        }
    }
}
