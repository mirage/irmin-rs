use crate::internal::*;

/// Wrapper around String_list Path type
pub struct Path<'a> {
    pub ptr: *mut IrminPath,
    pub repo: UntypedRepo<'a>,
}

impl<'a> Drop for Path<'a> {
    fn drop(&mut self) {
        unsafe { irmin_path_free(self.ptr) }
    }
}

impl<'a> PartialEq for Path<'a> {
    fn eq(&self, other: &Path<'a>) -> bool {
        unsafe { irmin_path_equal(self.repo.ptr, self.ptr, other.ptr) }
    }
}

impl<'a> Path<'a> {
    /// Create a path from a string
    pub fn from_str<T: Contents>(repo: &'a Repo<T>, s: impl AsRef<str>) -> Result<Path, Error> {
        unsafe {
            let s = s.as_ref();
            let ptr = irmin_path_of_string(repo.ptr, s.as_ptr() as *mut _, s.len() as i64);
            check!(repo.ptr, ptr);
            Ok(Path {
                ptr,
                repo: UntypedRepo::new(repo),
            })
        }
    }

    /// Create a path from a slice
    pub fn new<T: Contents>(repo: &'a Repo<T>, s: &[impl AsRef<str>]) -> Result<Path<'a>, Error> {
        let s: Vec<_> = s.iter().map(cstring).collect();
        let mut t: Vec<_> = s.iter().map(|x| x.as_ptr() as *mut u8).collect();
        t.push(std::ptr::null_mut());
        let ptr = unsafe { irmin_path(repo.ptr, t.as_ptr() as *mut _) };
        check!(repo.ptr, ptr);
        Ok(Path {
            ptr,
            repo: UntypedRepo::new(repo),
        })
    }

    /// Create an empty path
    pub fn empty<T: Contents>(repo: &'a Repo<T>) -> Result<Path<'a>, Error> {
        let ptr = unsafe { irmin_path_empty(repo.ptr as *mut _) };
        check!(repo.ptr, ptr);
        Ok(Path {
            ptr,
            repo: UntypedRepo::new(repo),
        })
    }

    /// Get path's parent path
    pub fn parent(&self) -> Result<Option<Path<'a>>, Error> {
        let ptr = unsafe { irmin_path_parent(self.repo.ptr, self.ptr) };
        check_opt!(self.repo.ptr, ptr);
        Ok(Some(Path {
            ptr,
            repo: self.repo.clone(),
        }))
    }

    /// Append to a path and return a new path
    pub fn append(&self, s: impl AsRef<str>) -> Result<Path<'a>, Error> {
        let s = s.as_ref();
        let ptr = unsafe {
            irmin_path_append(
                self.repo.ptr,
                self.ptr,
                s.as_ptr() as *mut _,
                s.len() as i64,
            )
        };
        check!(self.repo.ptr, ptr);
        Ok(Path {
            ptr,
            repo: self.repo.clone(),
        })
    }

    /// Append two paths
    pub fn append_path(&self, s: &Path) -> Result<Path<'a>, Error> {
        let ptr = unsafe { irmin_path_append_path(self.repo.ptr, self.ptr, s.ptr) };
        check!(self.repo.ptr, ptr);
        Ok(Path {
            ptr,
            repo: self.repo.clone(),
        })
    }

    /// Convert a path to String
    pub fn to_string(&self) -> Result<String, Error> {
        let ptr = unsafe { irmin_path_to_string(self.repo.ptr, self.ptr) };
        let s = IrminString::wrap(ptr);
        s.map(|x| x.into())
    }
}
