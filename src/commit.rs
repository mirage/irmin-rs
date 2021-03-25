use crate::{irmin, Info, Type};

#[derive(Debug, Clone, PartialEq, Type)]
pub struct Commit<Hash: Type> {
    pub node: Hash,
    pub parents: Vec<Hash>,
    pub info: Info,
}

impl<Hash: Type + Clone> Commit<Hash> {
    pub fn new(node: Hash, parents: impl AsRef<[Hash]>, info: Info) -> Commit<Hash> {
        Commit {
            node,
            parents: parents.as_ref().to_vec(),
            info,
        }
    }
}
