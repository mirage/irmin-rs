use crate::{irmin, Info, Type};

#[derive(Debug, Clone, PartialEq, Type)]
pub struct Commit<H: Type> {
    pub node: H,
    pub parents: Vec<H>,
    pub info: Info,
}

impl<H: Type + Clone> Commit<H> {
    pub fn new(node: H, parents: impl AsRef<[H]>, info: Info) -> Commit<H> {
        Commit {
            node,
            parents: parents.as_ref().to_vec(),
            info,
        }
    }
}
