use crate::internal::*;

/// Wrapper around irmin trees
pub struct Metadata<'a> {
    pub ptr: *mut IrminMetadata,
    pub repo: UntypedRepo<'a>,
}

impl<'a> Drop for Metadata<'a> {
    fn drop(&mut self) {
        unsafe { irmin_metadata_free(self.ptr) }
    }
}

impl<'a> Metadata<'a> {
    pub fn default<T: Contents>(repo: &'a Repo<T>) -> Result<Metadata<'a>, Error> {
        let m = unsafe { irmin_metadata_default(repo.ptr) };
        check!(m);
        Ok(Metadata {
            ptr: m,
            repo: UntypedRepo::new(repo),
        })
    }
}
