use ocaml_interop::*;

use crate::bindings::*;

wrapper!(Repo);

ocaml! {
    fn repo(config: Config) -> Repo;
}

impl Repo {
    pub fn new(ctx: &mut Context, cfg: &Config) -> Repo {
        let cr = &mut ctx.rt;
        let cfg = cfg.to_ocaml(cr).root();
        let x: BoxRoot<Repo> = repo(cr, &cfg);
        x.to_rust(cr)
    }
}
