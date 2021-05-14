use crate::{Key, Type};

macro_rules! wrapper {
    ($x: ident) => {
        pub struct $x(BoxRoot<$x>);

        unsafe impl FromOCaml<$x> for $x {
            fn from_ocaml(v: OCaml<'_, $x>) -> Self {
                $x(v.root())
            }
        }

        unsafe impl ToOCaml<$x> for $x {
            fn to_ocaml<'a>(&self, rt: &'a mut OCamlRuntime) -> OCaml<'a, $x> {
                self.0.get(rt)
            }
        }
    };
}

mod config;
mod context;
mod repo;
mod store;
mod tree;

pub use config::*;
pub use context::*;
pub use repo::*;
pub use store::*;
pub use tree::*;

#[cfg(test)]
mod tests {
    use crate::bindings::*;

    #[test]
    fn basic() {
        let mut ctx = Builder::new().with_store_type("mem").build();
        let cfg = Config::new(&mut ctx, "test123");
        let repo = Repo::new(&mut ctx, &cfg);
        let master = Store::master(&mut ctx, &repo);
        let key = Key::new(&["a", "b", "c"]);
        let x = master.find(&mut ctx, &key);
        assert!(x.is_none());

        master.set(&mut ctx, &key, "123", "test");

        let x = master.find(&mut ctx, &key).unwrap();
        assert!(x.as_str() == "123");

        let t = Tree::empty(&mut ctx);
        let foo_key = Key::new(&["foo"]);
        assert!(!t.mem(&mut ctx, &foo_key));
        let t = t.add(&mut ctx, &foo_key, "bar");
        assert!(t.mem(&mut ctx, &foo_key));
    }
}
