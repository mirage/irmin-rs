use crate::internal::*;

/// Wrapper around Irmin.S
pub struct Store<'a, T: Contents> {
    pub ptr: *mut Irmin,
    pub repo: &'a Repo<T>,
}

impl<'a, T: Contents> Store<'a, T> {
    /// Open the main branch of a store
    pub fn new(repo: &'a Repo<T>) -> Result<Store<'a, T>, Error> {
        unsafe {
            let ptr = irmin_main(repo.ptr);
            check!(ptr);
            Ok(Store { ptr, repo })
        }
    }

    /// Specify the branch to open
    pub fn of_branch(repo: &'a Repo<T>, branch: impl AsRef<str>) -> Result<Store<'a, T>, Error> {
        let branch = cstring(branch);
        unsafe {
            let ptr = irmin_of_branch(repo.ptr, branch.as_ptr() as *mut _);
            check!(ptr);
            Ok(Store { ptr, repo })
        }
    }

    /// Set a value, creating a new commit
    pub fn set(&mut self, path: &Path, value: &T, info: Info) -> Result<bool, Error> {
        let value = value.to_value()?;
        unsafe {
            let r = irmin_set(self.ptr, path.ptr, value.ptr, info.ptr);
            check!(r, false);
            Ok(r)
        }
    }

    /// Set a value if `old` matches the current value
    pub fn test_and_set(
        &mut self,
        path: &Path,
        old: Option<&T>,
        value: Option<&T>,
        info: Info,
    ) -> Result<bool, Error> {
        let old = match old {
            Some(value) => Some(value.to_value()?),
            None => None,
        };
        let value = match value {
            Some(value) => Some(value.to_value()?),
            None => None,
        };
        unsafe {
            let r = irmin_test_and_set(
                self.ptr,
                path.ptr,
                old.map(|x| x.ptr).unwrap_or_else(|| std::ptr::null_mut()),
                value.map(|x| x.ptr).unwrap_or_else(|| std::ptr::null_mut()),
                info.ptr,
            );
            check!(r, false);
            Ok(r)
        }
    }

    /// Set a tree, creating a new commit
    pub fn set_tree(&mut self, path: &Path, tree: &Tree<T>, info: Info) -> Result<bool, Error> {
        unsafe {
            let r = irmin_set_tree(self.ptr, path.ptr, tree.ptr, info.ptr);
            check!(r, false);
            Ok(r)
        }
    }

    /// Set a tree if `old` matches the current tree
    pub fn test_and_set_tree(
        &mut self,
        path: &Path,
        old: Option<&Tree<T>>,
        tree: Option<&Tree<T>>,
        info: Info,
    ) -> Result<bool, Error> {
        unsafe {
            let r = irmin_test_and_set_tree(
                self.ptr,
                path.ptr,
                old.map(|x| x.ptr).unwrap_or_else(|| std::ptr::null_mut()),
                tree.map(|x| x.ptr).unwrap_or_else(|| std::ptr::null_mut()),
                info.ptr,
            );
            check!(r, false);
            Ok(r)
        }
    }

    /// Find the value associated with the given path
    pub fn find(&self, path: &Path) -> Result<Option<T>, Error> {
        let r = unsafe { irmin_find(self.ptr, path.ptr) };
        check!(r);
        if r.is_null() {
            return Ok(None);
        }
        let ty = T::ty()?;
        let v = Value { ptr: r, ty };
        let v = T::from_value(&v)?;
        Ok(Some(v))
    }

    /// Find the tree associated with the given path
    pub fn find_tree(&self, path: &Path) -> Result<Option<Tree<T>>, Error> {
        unsafe {
            let ptr = irmin_find_tree(self.ptr, path.ptr);
            check!(ptr);
            if ptr.is_null() {
                return Ok(None);
            }
            let x = Tree {
                ptr,
                repo: self.repo,
            };
            Ok(Some(x))
        }
    }

    /// Check for the existence of a value at the given path
    pub fn mem(&self, path: &Path) -> bool {
        unsafe { irmin_mem(self.ptr, path.ptr) }
    }

    /// Check for the existence of a tree at the given path
    pub fn mem_tree(&self, path: &Path) -> bool {
        unsafe { irmin_mem_tree(self.ptr, path.ptr) }
    }

    /// Remove the tree or value associated with the given path
    pub fn remove(&mut self, path: &Path, info: Info) -> bool {
        unsafe { irmin_remove(self.ptr, path.ptr, info.ptr) }
    }

    /// Get current head commit
    pub fn head(&self) -> Option<Commit<'a>> {
        let ptr = unsafe { irmin_get_head(self.ptr) };
        if ptr.is_null() {
            return None;
        }

        Some(Commit {
            ptr,
            repo: UntypedRepo::new(&self.repo),
        })
    }

    /// Set head commit
    pub fn set_head(&mut self, c: &Commit) {
        unsafe { irmin_set_head(self.ptr, c.ptr) }
    }

    /// Update current branch to the specified commit
    pub fn fast_forward(&mut self, c: &Commit) -> bool {
        unsafe { irmin_fast_forward(self.ptr, c.ptr) }
    }

    /// Merge with another branch
    pub fn merge_with_branch(&mut self, branch: impl AsRef<str>, info: Info) -> bool {
        let branch = cstring(branch);
        unsafe { irmin_merge_with_branch(self.ptr, branch.as_ptr() as *mut _, info.ptr) }
    }

    /// Merge with another commit
    pub fn merge_with_commit(&mut self, commit: &Commit, info: Info) -> bool {
        unsafe { irmin_merge_with_commit(self.ptr, commit.ptr, info.ptr) }
    }

    /// Merge with another store
    pub fn merge(&mut self, store: &Store<T>, info: Info) -> bool {
        unsafe { irmin_merge_into(self.ptr, store.ptr, info.ptr) }
    }

    /// List paths
    pub fn list(&self, path: &Path) -> Result<Vec<Path>, Error> {
        let p = unsafe { irmin_list(self.ptr, path.ptr) };
        check!(p);
        let len = unsafe { irmin_path_list_length(self.repo.ptr, p) };
        let mut dest = Vec::new();
        for i in 0..len {
            let path = unsafe { irmin_path_list_get(self.repo.ptr, p, i) };
            if path.is_null() {
                continue;
            }
            dest.push(Path {
                ptr: path,
                repo: UntypedRepo::new(&self.repo),
            })
        }

        unsafe { irmin_path_list_free(p) }

        Ok(dest)
    }
}

impl<'a, T: Contents> Drop for Store<'a, T> {
    fn drop(&mut self) {
        unsafe { irmin_free(self.ptr) }
    }
}
