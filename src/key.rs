use crate::Type;
use ocaml_interop::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Key(Vec<String>);

impl Type for Key {
    fn encode_bin<W: std::io::Write>(&self, dest: &mut W) -> std::io::Result<usize> {
        self.0.encode_bin(dest)
    }

    fn decode_bin<R: std::io::Read>(src: &mut R) -> std::io::Result<Self> {
        let x = Vec::<String>::decode_bin(src)?;
        Ok(Key(x))
    }
}

unsafe impl FromOCaml<String> for Key {
    fn from_ocaml(v: OCaml<'_, String>) -> Self {
        let mut bytes = v.as_bytes();
        Key::decode_bin(&mut bytes).expect("Invalid key argument passed to Rust")
    }
}

unsafe impl ToOCaml<String> for Key {
    fn to_ocaml<'a>(&self, rt: &'a mut OCamlRuntime) -> OCaml<'a, String> {
        let mut data = Vec::new();
        self.encode_bin(&mut data)
            .expect("Invalid key argument passed to OCaml");
        data.to_ocaml(rt)
    }
}

impl Key {
    pub fn new<'a>(a: impl AsRef<[&'a str]>) -> Key {
        Key(a
            .as_ref()
            .iter()
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(x.to_string())
                }
            })
            .collect())
    }

    pub fn empty() -> Key {
        Key(vec![])
    }

    pub fn push(&mut self, p: impl Into<String>) {
        let p = p.into();
        if !p.is_empty() {
            self.0.push(p.into())
        }
    }

    pub fn pop(&mut self) -> Option<String> {
        self.0.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn to_string(&self) -> String {
        self.0.join("/")
    }

    pub fn from_string(s: impl AsRef<str>) -> Key {
        Key::new(s.as_ref().split("/").collect::<Vec<_>>())
    }
}
