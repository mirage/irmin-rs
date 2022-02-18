use crate::internal::*;

pub struct Remote<'a> {
    pub ptr: *mut IrminRemote,
    #[allow(dead_code)]
    repo: UntypedRepo<'a>,
}

impl<'a> Remote<'a> {
    /// Create `Remote` from an existing store
    pub fn store<T: Contents>(store: &'a Store<T>) -> Result<Remote<'a>, Error> {
        let ptr = unsafe { irmin_remote_store(store.ptr) };
        check!(store.repo.ptr, ptr);
        Ok(Remote {
            ptr,
            repo: UntypedRepo::new(store.repo),
        })
    }

    /// Remote from URL
    pub fn url<T: Contents>(repo: &'a Repo<T>, s: impl AsRef<str>) -> Result<Remote<'a>, Error> {
        let mut s = cstring(s.as_ref());
        let ptr = unsafe { irmin_remote(repo.ptr, s.as_mut_ptr() as *mut _) };
        check!(repo.ptr, ptr);
        Ok(Remote {
            ptr,
            repo: UntypedRepo::new(repo),
        })
    }

    /// Remote from URL with basic auth
    pub fn url_with_auth<T: Contents>(
        repo: &'a Repo<T>,
        s: impl AsRef<str>,
        user: impl AsRef<str>,
        token: impl AsRef<str>,
    ) -> Result<Remote<'a>, Error> {
        let mut s = cstring(s.as_ref());
        let mut user = cstring(user.as_ref());
        let mut token = cstring(token.as_ref());
        let ptr = unsafe {
            irmin_remote_with_auth(
                repo.ptr,
                s.as_mut_ptr() as *mut _,
                user.as_mut_ptr() as *mut _,
                token.as_mut_ptr() as *mut _,
            )
        };
        check!(repo.ptr, ptr);
        Ok(Remote {
            ptr,
            repo: UntypedRepo::new(repo),
        })
    }
}

impl<'a> Drop for Remote<'a> {
    fn drop(&mut self) {
        unsafe { irmin_remote_free(self.ptr) }
    }
}
