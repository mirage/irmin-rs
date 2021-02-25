use ocaml_interop::*;

pub enum Contents {
    String,
    Json,
    JsonValue,
}

ocaml! {
    fn config(root: String) -> Config;
    fn repo(config: Config) -> Repo;
    fn store_master(repo: Repo) -> Store;
    fn store_mem(store: Store, key: String) -> bool;
    fn store_remove(store: Store, key: String, message: String);
    fn store_find(store: Store, key: String) -> Option<OCamlBytes>;
    fn store_set(store: Store, key: String, value: String, message: String);
}

pub struct Context {
    rt: std::cell::RefCell<OCamlRuntime>,
}

impl Context {
    pub fn new() -> Context {
        let rt = std::cell::RefCell::new(OCamlRuntime::init());
        Context { rt }
    }
}

macro_rules! wrapper {
    ($x: ident) => {
        pub struct $x(RawOCaml);

        unsafe impl FromOCaml<$x> for $x {
            fn from_ocaml(v: OCaml<'_, $x>) -> Self {
                unsafe { $x(v.raw()) }
            }
        }

        unsafe impl ToOCaml<$x> for $x {
            fn to_ocaml<'a>(&self, rt: &'a mut OCamlRuntime) -> OCaml<'a, $x> {
                unsafe { OCaml::new(rt, self.0) }
            }
        }
    };
}

wrapper!(Config);

impl Config {
    pub fn new(ctx: &Context, root: impl AsRef<str>) -> Config {
        let cr = &mut ctx.rt.borrow_mut();
        ocaml_frame!(cr, (r), {
            let root = to_ocaml!(cr, root.as_ref(), r);
            let x: OCaml<'_, Config> = config(cr, &root);
            x.to_rust()
        })
    }
}

wrapper!(Repo);

impl Repo {
    pub fn new(ctx: &Context, cfg: &Config) -> Repo {
        let cr = &mut ctx.rt.borrow_mut();
        ocaml_frame!(cr, (r), {
            let cfg = to_ocaml!(cr, cfg, r);
            let x: OCaml<'_, Repo> = repo(cr, &cfg);
            x.to_rust()
        })
    }
}

wrapper!(Store);

impl Store {
    pub fn master(ctx: &Context, repo: &Repo) -> Store {
        let cr = &mut ctx.rt.borrow_mut();
        ocaml_frame!(cr, (r), {
            let repo = to_ocaml!(cr, repo, r);
            let x: OCaml<'_, Store> = store_master(cr, &repo);
            x.to_rust()
        })
    }

    pub fn mem(&self, ctx: &Context, key: impl AsRef<str>) -> bool {
        let cr = &mut ctx.rt.borrow_mut();
        ocaml_frame!(cr, (s, r), {
            let store = to_ocaml!(cr, self, s);
            let key = to_ocaml!(cr, key.as_ref(), r);
            let x: OCaml<'_, bool> = store_mem(cr, &store, &key);
            x.to_rust()
        })
    }

    pub fn find(&self, ctx: &Context, key: impl AsRef<str>) -> Option<String> {
        let cr = &mut ctx.rt.borrow_mut();
        ocaml_frame!(cr, (s, r), {
            let store = to_ocaml!(cr, self, s);
            let key = to_ocaml!(cr, key.as_ref(), r);
            let x = store_find(cr, &store, &key);
            x.to_rust()
        })
    }

    pub fn remove(&self, ctx: &Context, key: impl AsRef<str>, info: impl AsRef<str>) {
        let cr = &mut ctx.rt.borrow_mut();
        ocaml_frame!(cr, (s, r, i), {
            let store = to_ocaml!(cr, self, s);
            let key = to_ocaml!(cr, key.as_ref(), r);
            let info = to_ocaml!(cr, info.as_ref(), i);
            let _: OCaml<'_, ()> = store_remove(cr, &store, &key, &info);
        })
    }

    pub fn set(
        &self,
        ctx: &Context,
        key: impl AsRef<str>,
        value: impl AsRef<[u8]>,
        message: impl AsRef<str>,
    ) {
        let cr = &mut ctx.rt.borrow_mut();
        ocaml_frame!(cr, (s, r, i, v), {
            let store = to_ocaml!(cr, self, s);
            let key = to_ocaml!(cr, key.as_ref(), r);
            let info = to_ocaml!(cr, message.as_ref(), i);
            let value = to_ocaml!(cr, value.as_ref(), v);
            let _: OCaml<'_, ()> = store_set(cr, &store, &key, &value, &info);
        })
    }
}

pub trait Type: Sized {
    fn to_string(&self) -> String;
    fn from_string(s: &str) -> Option<Self>;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basic() {
        let ctx = Context::new();
        let cfg = Config::new(&ctx, "test123");
        let repo = Repo::new(&ctx, &cfg);
        let master = Store::master(&ctx, &repo);
        let x = master.find(&ctx, "/a/b/c");
        assert!(x.is_none());

        master.set(&ctx, "/a/b/c", "123", "test");

        let x = master.find(&ctx, "/a/b/c").unwrap();
        assert!(x.as_str() == "123");
    }
}
