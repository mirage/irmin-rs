use ocaml_interop::*;

use crate::bindings::*;

wrapper!(Store);

ocaml! {
    fn store_master(repo: Repo) -> Store;
    fn store_mem(store: Store, key: String) -> bool;
    fn store_remove(store: Store, key: String, message: String);
    fn store_find(store: Store, key: String) -> Option<OCamlBytes>;
    fn store_set(store: Store, key: String, value: OCamlBytes, message: String);
}

impl Store {
    pub fn master(ctx: &Context, repo: &Repo) -> Store {
        let cr = &mut ctx.rt.borrow_mut();
        let repo = repo.to_ocaml(cr).root();
        let x: BoxRoot<Store> = store_master(cr, &repo);
        x.to_rust(cr)
    }

    pub fn mem(&self, ctx: &Context, key: &Key) -> bool {
        let cr = &mut ctx.rt.borrow_mut();
        let store = self.to_ocaml(cr).root();
        let key = key.to_ocaml(cr).root();
        let x: BoxRoot<bool> = store_mem(cr, &store, &key);
        x.to_rust(cr)
    }

    pub fn find(&self, ctx: &Context, key: &Key) -> Option<String> {
        let cr = &mut ctx.rt.borrow_mut();
        let store = self.to_ocaml(cr).root();
        let key = key.to_ocaml(cr).root();
        let x = store_find(cr, &store, &key);
        x.to_rust(cr)
    }

    pub fn remove(&self, ctx: &Context, key: &Key, msg: impl AsRef<str>) {
        let cr = &mut ctx.rt.borrow_mut();
        let store = self.to_ocaml(cr).root();
        let key = key.to_ocaml(cr).root();
        let info = msg.as_ref().to_ocaml(cr).root();
        let _: BoxRoot<()> = store_remove(cr, &store, &key, &info);
    }

    pub fn set(&self, ctx: &Context, key: &Key, value: impl AsRef<[u8]>, msg: impl AsRef<str>) {
        let cr = &mut ctx.rt.borrow_mut();
        let store = self.to_ocaml(cr).root();
        let key = key.to_ocaml(cr).root();
        let info = msg.as_ref().to_ocaml(cr).root();
        let value = value.as_ref().to_ocaml(cr).root();
        let _: BoxRoot<()> = store_set(cr, &store, &key, &value, &info);
    }
}
