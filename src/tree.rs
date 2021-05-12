use std::collections::BTreeMap;

use crate as irmin;
use crate::{Hash, Type};

#[derive(Debug, Clone, Type, PartialEq)]
pub enum Tree<T: Type, H: Hash> {
    Hash(H),
    Id(isize),
    Concrete(Concrete<T>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Concrete<T> {
    Tree(BTreeMap<String, Concrete<T>>),
    Contents(T),
}

impl<T: Type, H: Hash> Tree<T, H> {
    pub fn empty() -> Self {
        Tree::Concrete(Concrete::empty())
    }
}

impl<T: Type> Type for Concrete<T> {
    fn encode_bin<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<usize> {
        match self {
            Concrete::Contents(v) => {
                let mut n = 1usize.encode_bin(w)?;
                n += v.encode_bin(w)?;
                Ok(n)
            }
            Concrete::Tree(t) => {
                let mut n = 0usize.encode_bin(w)?;
                n += t.encode_bin(w)?;
                Ok(n)
            }
        }
    }

    fn decode_bin<R: std::io::Read>(r: &mut R) -> std::io::Result<Self> {
        let header = usize::decode_bin(r)?;
        match header {
            0 => Ok(Concrete::Tree(BTreeMap::decode_bin(r)?)),
            1 => Ok(Concrete::Contents(T::decode_bin(r)?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid Tree format",
            )),
        }
    }
}

impl<T: Type> Concrete<T> {
    pub fn empty() -> Self {
        Concrete::Tree(BTreeMap::new())
    }

    pub fn is_tree(&self) -> bool {
        match self {
            Concrete::Tree(_) => true,
            _ => false,
        }
    }

    pub fn is_contents(&self) -> bool {
        match self {
            Concrete::Contents(_) => true,
            _ => false,
        }
    }

    pub fn add(&mut self, key: impl Into<String>, value: T) {
        match self {
            Concrete::Tree(t) => {
                let value = Concrete::Contents(value);
                t.insert(key.into(), value);
            }
            Concrete::Contents(_) => {
                *self = Self::empty();
                self.add(key, value);
            }
        }
    }

    pub fn add_tree(&mut self, key: impl Into<String>, tree: Concrete<T>) {
        match self {
            Concrete::Tree(t) => {
                t.insert(key.into(), tree);
            }
            Concrete::Contents(_) => {
                *self = Self::empty();
                self.add_tree(key, tree);
            }
        }
    }

    pub fn remove(&mut self, key: impl AsRef<str>) {
        match self {
            Concrete::Tree(t) => {
                t.remove(key.as_ref());
            }
            _ => (),
        }
    }

    pub fn mem(&self, key: impl AsRef<str>) -> bool {
        match self {
            Concrete::Tree(t) => t.contains_key(key.as_ref()),
            _ => false,
        }
    }

    pub fn mem_tree(&self, key: impl AsRef<str>) -> bool {
        match self {
            Concrete::Tree(t) => match t.get(key.as_ref()) {
                Some(Concrete::Tree(_)) => true,
                _ => false,
            },
            _ => false,
        }
    }
}
