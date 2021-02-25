use ocaml::{FromValue, IntoValue, Runtime, Value};

#[derive(ocaml::IntoValue)]
pub enum Contents {
    String,
    Json,
    JsonValue,
}

pub struct Context {
    rt: Runtime,
}

impl Context {
    pub fn new() -> Context {
        let rt = ocaml::Runtime::init();
        Context { rt }
    }
}

pub struct Config(ocaml::Value);

impl Config {
    pub fn new(ctx: &Context, root: impl AsRef<str>) -> Result<Config, ocaml::Error> {
        unsafe {
            let f: Value = ocaml::Value::named("config").unwrap();
            let cfg = f.call(&ctx.rt, root.as_ref().into_value(&ctx.rt))?;
            Ok(Config(cfg))
        }
    }
}

pub struct Repo(ocaml::Value);

impl Repo {
    pub fn new(ctx: &Context, cfg: &Config) -> Result<Repo, ocaml::Error> {
        unsafe {
            let f: Value = ocaml::Value::named("repo").unwrap();
            let repo = f.call(&ctx.rt, cfg.0)?;
            Ok(Repo(repo))
        }
    }
}

pub struct Store(ocaml::Value);

impl Store {
    pub fn master(ctx: &Context, repo: &Repo) -> Result<Store, ocaml::Error> {
        unsafe {
            let f: Value = ocaml::Value::named("store_master").unwrap();
            let store = f.call(&ctx.rt, repo.0)?;
            Ok(Store(store))
        }
    }

    pub fn mem(&self, ctx: &Context, key: impl AsRef<str>) -> Result<bool, ocaml::Error> {
        unsafe {
            let f: Value = ocaml::Value::named("store_mem").unwrap();
            let ok = f.call2(&ctx.rt, self.0, key.as_ref().into_value(&ctx.rt))?;
            Ok(bool::from_value(ok))
        }
    }

    pub fn find(&self, ctx: &Context, key: impl AsRef<str>) -> Result<Option<&[u8]>, ocaml::Error> {
        unsafe {
            let f: Value = ocaml::Value::named("store_find").unwrap();
            let ok = f.call2(&ctx.rt, self.0, key.as_ref().into_value(&ctx.rt))?;
            Ok(FromValue::from_value(ok))
        }
    }

    pub fn set(
        &self,
        ctx: &Context,
        key: impl AsRef<str>,
        value: impl AsRef<[u8]>,
        message: impl AsRef<str>,
    ) -> Result<(), ocaml::Error> {
        unsafe {
            let f: Value = ocaml::Value::named("store_set").unwrap();
            f.call_n(
                &ctx.rt,
                &[
                    self.0,
                    key.as_ref().into_value(&ctx.rt),
                    value.as_ref().into_value(&ctx.rt),
                    message.as_ref().into_value(&ctx.rt),
                ],
            )?;
            Ok(())
        }
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
        let cfg = Config::new(&ctx, "test123").unwrap();
        let repo = Repo::new(&ctx, &cfg).unwrap();
        let master = Store::master(&ctx, &repo).unwrap();
        let x = master.find(&ctx, "/a/b/c").unwrap();
        assert!(x.is_none());

        master.set(&ctx, "/a/b/c", "123", "test").unwrap();

        let x = master.find(&ctx, "/a/b/c").unwrap().unwrap();
        assert!(x == b"123");
    }
}
