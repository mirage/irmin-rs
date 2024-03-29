use crate::internal::*;

/// Wrapper around Irmin repo
pub struct Repo<T: Contents> {
    pub config: Config<T>,
    pub ptr: *mut IrminRepo,
}

impl<T: Contents> Repo<T> {
    /// Create a new repo from the given config
    pub fn new(config: Config<T>) -> Result<Repo<T>, Error> {
        unsafe {
            let ptr = irmin_repo_new(config.ptr);
            if ptr.is_null() {
                return Err(Error::NullPtr);
            }
            Ok(Repo { config, ptr })
        }
    }

    /// Get a list of all branches
    pub fn branches(&self) -> Result<Vec<IrminString>, Error> {
        let b = unsafe { irmin_repo_branches(self.ptr) };
        check!(self.ptr, b);
        let mut dest = Vec::new();
        let n = unsafe { irmin_branch_array_length(self.ptr, b) };
        for i in 0..n {
            let p = unsafe { irmin_branch_array_get(self.ptr, b, i) };
            if let Ok(s) = IrminString::wrap(p) {
                dest.push(s);
            }
        }
        unsafe { irmin_branch_array_free(b) };
        Ok(dest)
    }

    /// Create a new path
    pub fn path(&self, s: &[impl AsRef<str>]) -> Result<Path, Error> {
        Path::new(self, s)
    }

    /// Create a new commit
    pub fn commit<'a>(
        &'a self,
        parents: impl AsRef<[&'a Commit<'a>]>,
        tree: &Tree<T>,
        info: Info,
    ) -> Result<Commit<'a>, Error> {
        Commit::new(self, parents, tree, info)
    }

    /// Create an empty tree
    pub fn tree(&self) -> Result<Tree<T>, Error> {
        Tree::new(self)
    }

    /// Create commit info
    pub fn info(&self, author: impl AsRef<str>, message: impl AsRef<str>) -> Result<Info, Error> {
        Info::new(self, author, message)
    }
}

impl<T: Contents> Drop for Repo<T> {
    fn drop(&mut self) {
        unsafe { irmin_repo_free(self.ptr) }
    }
}
