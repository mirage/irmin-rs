use crate::internal::*;

pub(crate) fn cstring(s: impl AsRef<str>) -> String {
    let mut s = s.as_ref().to_string();
    s.push('\0');
    s
}

#[derive(Clone)]
pub struct UntypedRepo<'a> {
    pub(crate) ptr: *mut IrminRepo,
    pub(crate) _t: std::marker::PhantomData<&'a ()>,
}

impl<'a> UntypedRepo<'a> {
    pub fn new<T: Contents>(repo: &'a Repo<T>) -> UntypedRepo<'a> {
        UntypedRepo {
            ptr: repo.ptr,
            _t: std::marker::PhantomData,
        }
    }
}
