use ocaml_interop::*;

use crate::bindings::*;

wrapper!(Tree);

ocaml! {
    fn tree_of_concrete(concrete: String) -> Tree;
    fn tree_to_concrete(t: Tree) -> OCamlBytes;
    fn tree_empty(unit: ()) -> Tree;
    fn tree_add(t: Tree, key: String, value: OCamlBytes) -> Tree;
    fn tree_mem(t: Tree, key: String) -> bool;
}

impl Tree {
    pub fn empty(ctx: &mut Context) -> Tree {
        let mut cr = &mut ctx.rt;
        let arg = ().to_ocaml(&mut cr).root();
        tree_empty(&mut cr, &arg).to_rust(&mut cr)
    }

    pub fn of_concrete<T: Type>(ctx: &mut Context, c: &crate::Concrete<T>) -> Tree {
        let mut cr = &mut ctx.rt;
        let mut dest = Vec::new();
        c.encode_bin(&mut dest).expect("Invalid Tree");
        let s = dest.to_ocaml(&mut cr).root();
        tree_of_concrete(&mut cr, &s).to_rust(&mut cr)
    }

    pub fn to_concrete<T: Type>(&self, ctx: &mut Context) -> crate::Concrete<T> {
        let mut cr = &mut ctx.rt;

        let tree = self.to_ocaml(&mut cr).root();
        let s = tree_to_concrete(&mut cr, &tree);
        let s: Vec<u8> = s.to_rust(&mut cr);
        Type::decode_bin(&mut s.as_slice()).expect("Invalid tree")
    }

    pub fn add(&self, ctx: &mut Context, key: &Key, value: impl AsRef<[u8]>) -> Tree {
        let cr = &mut ctx.rt;
        let tree = self.to_ocaml(cr).root();
        let key = key.to_ocaml(cr).root();
        let value = value.as_ref().to_ocaml(cr).root();
        tree_add(cr, &tree, &key, &value).to_rust(cr)
    }

    pub fn mem(&self, ctx: &mut Context, key: &Key) -> bool {
        let cr = &mut ctx.rt;
        let tree = self.to_ocaml(cr).root();
        let key = key.to_ocaml(cr).root();
        tree_mem(cr, &tree, &key).to_rust(cr)
    }
}
