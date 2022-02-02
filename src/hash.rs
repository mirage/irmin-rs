use crate::internal::*;

/// Wrapper around Irmin hash type
pub struct Hash<'a> {
    pub ptr: *mut IrminHash,
    pub(crate) repo: UntypedRepo<'a>,
}

impl<'a> PartialEq for Hash<'a> {
    fn eq(&self, other: &Hash<'a>) -> bool {
        unsafe { irmin_hash_equal(self.repo.ptr, self.ptr, other.ptr) }
    }
}

impl<'a> Hash<'a> {
    /// Convert from string to Hash
    pub fn of_string<T: Contents>(
        repo: &'a Repo<T>,
        s: impl AsRef<str>,
    ) -> Result<Hash<'a>, Error> {
        let s = s.as_ref();
        let ptr = unsafe { irmin_hash_of_string(repo.ptr, s.as_ptr() as *mut _, s.len() as i64) };
        check!(repo.ptr, ptr);
        Ok(Hash {
            ptr,
            repo: UntypedRepo::new(repo),
        })
    }

    /// Convert from Hash to String
    pub fn to_string<T: Contents>(&self) -> Result<String, Error> {
        let s = unsafe { irmin_hash_to_string(self.repo.ptr, self.ptr) };
        IrminString::wrap(s).map(|x| x.into())
    }
}

impl<'a> Drop for Hash<'a> {
    fn drop(&mut self) {
        unsafe { irmin_hash_free(self.ptr) }
    }
}
