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
        let ctx = Context::new(Generator::new().with_store_type("mem"));
        let cfg = Config::new(&ctx, "test123");
        let repo = Repo::new(&ctx, &cfg);
        let master = Store::master(&ctx, &repo);
        let key = Key::new(&["a", "b", "c"]);
        let x = master.find(&ctx, &key);
        assert!(x.is_none());

        master.set(&ctx, &key, "123", "test");

        let x = master.find(&ctx, &key).unwrap();
        assert!(x.as_str() == "123");

        let t = Tree::empty(&ctx);
        let foo_key = Key::new(&["foo"]);
        assert!(!t.mem(&ctx, &foo_key));
        let t = t.add(&ctx, &foo_key, "bar");
        assert!(t.mem(&ctx, &foo_key));
    }
}
