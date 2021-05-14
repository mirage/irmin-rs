use ocaml_interop::*;

use crate::bindings::*;

wrapper!(Config);

ocaml! {
    fn config(root: String) -> Config;
}

impl Config {
    pub fn new(ctx: &mut Context, root: impl AsRef<str>) -> Config {
        let cr = &mut ctx.rt;
        let root = root.as_ref().to_ocaml(cr).root();
        let x: BoxRoot<Config> = config(cr, &root);
        x.to_rust(cr)
    }
}
