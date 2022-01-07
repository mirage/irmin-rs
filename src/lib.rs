/// Set internal log level
pub fn set_log_level(s: Option<&str>) {
    let s = s.map(internal::cstring);
    unsafe {
        bindings::irmin_log_level(
            s.map(|x| x.as_ptr() as *mut _)
                .unwrap_or_else(|| std::ptr::null_mut()),
        )
    }
}

pub fn error_msg() -> Option<IrminString> {
    let s = unsafe { bindings::irmin_error_msg() };
    if s.is_null() {
        return None;
    }
    match IrminString::wrap(s) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

#[macro_use]
pub mod bindings;

mod commit;
mod config;
mod hash;
mod info;
mod irmin_string;
mod path;
mod repo;
mod store;
mod tree;
mod ty;
mod util;
mod value;

pub(crate) mod prelude {
    pub use crate::commit::Commit;
    pub use crate::config::{Config, ContentType, Contents, HashType};
    pub use crate::hash::Hash;
    pub use crate::info::Info;
    pub use crate::irmin_string::IrminString;
    pub use crate::path::Path;
    pub use crate::repo::Repo;
    pub use crate::store::Store;
    pub use crate::tree::Tree;
    pub use crate::ty::Type;
    pub use crate::value::Value;
    pub use crate::Error;

    pub type Json = serde_json::Map<String, serde_json::Value>;
    pub type JsonValue = serde_json::Value;
    pub use serde_json::json;
}

pub(crate) mod internal {
    pub use crate::bindings::*;
    pub use crate::prelude::*;
    pub use crate::util::*;
}

pub use crate::prelude::*;

#[derive(Debug)]
pub enum Error {
    NullPtr,
    Exc(IrminString),
    Json(serde_json::Error),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Json(e)
    }
}

pub fn raise_error<T>(x: T) -> Result<T, Error> {
    match error_msg() {
        Some(err) => Err(Error::Exc(err)),
        None => Ok(x),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_store() -> Result<(), Error> {
        let config = Config::<serde_json::Value>::git_mem()?;
        let repo = Repo::new(config)?;
        let mut store = Store::new(&repo)?;

        let info = repo.info("irmin", "set")?;
        let path = Path::from_str(&repo, "foo/bar")?;
        let value = serde_json::json!({
            "a": 1,
            "b": 2,
            "c": 3,
        });
        assert!(store.set(&path, &value, info)?);

        let head = store.head().unwrap();
        assert!(head.parents()?.len() == 0);

        let s = store.find(&path)?;
        assert!(s.unwrap() == value);

        let path1 = path.parent().unwrap();
        assert!(store.mem_tree(&path1));

        let x = store.find_tree(&path1)?;
        assert!(x.is_some());

        let path2 = repo.path(&["bar"])?;
        let y = x.unwrap().find(&path2)?;
        assert!(y.unwrap() == value);

        let value1 = serde_json::json!({
            "a": 4,
            "b": 5,
            "c": 6,
        });

        let info = Info::new(&repo, "irmin", "set")?;
        assert!(store.set(&path, &value1, info)?);

        let head1 = store.head().unwrap();
        assert!(head1.parents()?.len() == 1);
        assert!(head1.parents()?[0] == head);

        Ok(())
    }

    #[test]
    fn test_tree() -> Result<(), Error> {
        let config = Config::<String>::git_mem()?;
        let repo = Repo::new(config)?;

        let mut tree = repo.tree()?;
        let abc = repo.path(&["a", "b", "c"])?;
        let ab = repo.path(&["a", "b"])?;

        let v = String::from("123");
        tree.add(&abc, &v)?;
        assert!(tree.mem(&abc));
        assert!(tree.mem_tree(&ab));

        Ok(())
    }
}
