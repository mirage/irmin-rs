use ocaml_interop::*;

ocaml! {
    fn store_gen(store: Option<String>, contents: Option<String>, hash: Option<String>);
}

pub struct Context {
    pub(crate) rt: std::cell::RefCell<OCamlRuntime>,
}

impl Context {
    fn new(gen: Builder) -> Context {
        let rt = std::cell::RefCell::new(OCamlRuntime::init());
        let ctx = Context { rt };
        gen.build_with_context(&ctx);
        ctx
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Builder {
    store: Option<String>,
    hash: Option<String>,
    contents: Option<String>,
}

impl Builder {
    pub fn new() -> Builder {
        Self::default()
    }

    pub fn with_store_type(mut self, s: impl Into<String>) -> Self {
        self.store = Some(s.into());
        self
    }

    pub fn with_content_type(mut self, s: impl Into<String>) -> Self {
        self.contents = Some(s.into());
        self
    }

    pub fn with_hash(mut self, s: impl Into<String>) -> Self {
        self.hash = Some(s.into());
        self
    }

    fn build_with_context(self, ctx: &Context) {
        let cr = &mut ctx.rt.borrow_mut();
        let store = self.store.to_ocaml(cr).root();
        let hash = self.hash.to_ocaml(cr).root();
        let contents = self.contents.to_ocaml(cr).root();
        store_gen(cr, &store, &hash, &contents);
    }

    pub fn build(self) -> Context {
        Context::new(self)
    }
}
