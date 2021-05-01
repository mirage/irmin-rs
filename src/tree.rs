use crate as irmin;
use crate::Type;

/*#[derive(Debug, Clone, Type)]
pub enum Concrete<T: Type> {
    Tree(Vec<(String, Concrete<T>)>),
    Contents(T, ()),
}*/

#[derive(Debug, Clone, Type)]
pub enum Tree<T: Type, Hash: Type> {
    Hash(Hash),
    Id(isize),
    Concrete(Concrete<T>),
}

#[derive(Debug, Clone)]
pub struct Concrete<T: Type> {
    encoded: Vec<u8>,
    _t: std::marker::PhantomData<T>,
}

impl<T: Type> Type for Concrete<T> {
    fn name() -> String {
        String::new()
    }

    fn encode_bin<W: std::io::Write>(&self, mut w: W) -> std::io::Result<usize> {
        let len = self.encoded.len();
        //let count = len.encode_bin(&mut w)?;
        w.write_all(&self.encoded)?;
        Ok(len)
    }

    fn decode_bin<R: std::io::Read>(mut r: R) -> std::io::Result<Self> {
        let header = usize::decode_bin(&mut r)?;
        let mut data = vec![0u8; header];
        r.read_exact(&mut data)?;
        Ok(Concrete {
            encoded: data,
            _t: std::marker::PhantomData,
        })
    }
}

impl<T: Type, Hash: Type> Tree<T, Hash> {
    pub fn empty() -> Self {
        Tree::Concrete(Concrete {
            encoded: vec![0, 0],
            _t: std::marker::PhantomData,
        })
    }
}

/*impl<T: Type> Type for Concrete<T> {
    fn encode_bin<W: std::io::Write>(&self, mut w: W) -> std::io::Result<usize> {
        match self {
            Concrete::Contents(v, ()) => {
                let mut n = 1usize.encode_bin(&mut w)?;
                n += v.encode_bin(w)?;
                Ok(n)
            }
            Concrete::Tree(t) => {
                let mut n = 0usize.encode_bin(&mut w)?;
                n += t.encode_bin(w)?;
                Ok(n)
            }
        }
    }

    fn decode_bin<R: std::io::Read>(mut r: R) -> std::io::Result<Self> {
        let header = usize::decode_bin(&mut r)?;
        if header == 0 {
            Ok(Concrete::Tree(BTreeMap::decode_bin(r)?))
        } else {
            Ok(Concrete::Contents(T::decode_bin(r)?, ()))
        }
    }
}*/

/*impl<T: Type> Concrete<T> {
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
            Concrete::Contents(_, _) => true,
            _ => false,
        }
    }

    pub fn add(&mut self, key: impl Into<String>, value: T) {
        match self {
            Concrete::Tree(t) => {
                let value = Concrete::Contents(value, ());
                t.insert(key.into(), value);
            }
            Concrete::Contents(_, _) => {
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
            Concrete::Contents(_, _) => {
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
}*/

/*impl<Hash: Type> Type for Tree<Hash> {
    fn encode_bin<W: std::io::Write>(&self, w: W) -> std::io::Result<usize> {}

    fn decode_bin<R: std::io::Read>(r: R) -> std::io::Result<Self> {}
}*/
