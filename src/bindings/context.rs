use crate::Hash;
use ocaml_interop::*;

ocaml! {
    fn store_gen(store: Option<String>, contents: Option<String>, hash: Option<String>);
}

/// `Context` is the entrypoint into the OCaml runtime, it is needed when calling functions
/// that call OCaml under the hood
pub struct Context {
    pub(crate) rt: OCamlRuntime,
}

impl Clone for Context {
    fn clone(&self) -> Context {
        let rt = OCamlRuntime::init();
        let ctx = Context { rt };
        ctx
    }
}

impl Context {
    fn new(gen: Builder) -> Context {
        let rt = OCamlRuntime::init();
        let mut ctx = Context { rt };
        gen.build_with_context(&mut ctx);
        ctx
    }
}

/// A `Builder` is used to create a `Context`, currently only one `Builder` per process is
/// supported
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Builder {
    store: Option<String>,
    hash: Option<String>,
    contents: Option<String>,
}

impl Builder {
    /// Instantiate a new `Builder` instance
    pub fn new() -> Builder {
        Self::default()
    }

    /// Set store type
    pub fn with_store_type(mut self, s: impl Into<String>) -> Self {
        self.store = Some(s.into());
        self
    }

    /// Set contents type
    pub fn with_content_type(mut self, s: impl Into<String>) -> Self {
        self.contents = Some(s.into());
        self
    }

    /// Set hash type
    pub fn with_hash<H: Hash>(mut self) -> Self {
        // TODO: ensure this matches the store hash type
        self.hash = Some(H::name().into());
        self
    }

    fn build_with_context(self, ctx: &mut Context) {
        let cr = &mut ctx.rt;
        let store = self.store.to_ocaml(cr).root();
        let hash = self.hash.to_ocaml(cr).root();
        let contents = self.contents.to_ocaml(cr).root();
        store_gen(cr, &store, &contents, &hash);
    }

    /// Build a new `Context` using the specified store type
    pub fn build(self) -> Context {
        Context::new(self)
    }
}
