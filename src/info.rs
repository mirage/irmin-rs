use crate::internal::*;

/// Wrapper around irmin commit info
pub struct Info<'a> {
    pub ptr: *mut IrminInfo,
    pub(crate) repo: UntypedRepo<'a>,
}

impl<'a> Info<'a> {
    /// Create new commit info
    pub fn new<T: Contents>(
        repo: &Repo<T>,
        author: impl AsRef<str>,
        message: impl AsRef<str>,
    ) -> Result<Info, Error> {
        let message = cstring(message);
        let author = cstring(author);
        let ptr = unsafe {
            irmin_info_new(
                repo.ptr,
                author.as_ptr() as *mut _,
                message.as_ptr() as *mut _,
            )
        };
        check!(repo.ptr, ptr);
        Ok(Info {
            ptr,
            repo: UntypedRepo::new(repo),
        })
    }

    /// Get date
    pub fn date(&self) -> i64 {
        unsafe { irmin_info_date(self.repo.ptr, self.ptr) }
    }

    /// Get author
    pub fn author(&self) -> Result<IrminString, Error> {
        let ptr = unsafe { irmin_info_author(self.repo.ptr, self.ptr) };
        IrminString::wrap(ptr)
    }

    /// Get message
    pub fn message(&self) -> Result<IrminString, Error> {
        let ptr = unsafe { irmin_info_message(self.repo.ptr, self.ptr) };
        IrminString::wrap(ptr)
    }
}

impl<'a> Drop for Info<'a> {
    fn drop(&mut self) {
        unsafe { irmin_info_free(self.ptr) }
    }
}
