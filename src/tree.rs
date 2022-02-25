use crate::internal::*;

/// Wrapper around irmin trees
pub struct Tree<'a, T: Contents> {
    pub ptr: *mut IrminTree,
    pub repo: UntypedRepo<'a>,
    pub(crate) _t: std::marker::PhantomData<T>,
}

impl<'a, T: Contents> Drop for Tree<'a, T> {
    fn drop(&mut self) {
        unsafe { irmin_tree_free(self.ptr) }
    }
}

impl<'a, T: Contents> PartialEq for Tree<'a, T> {
    fn eq(&self, other: &Tree<'a, T>) -> bool {
        unsafe { irmin_tree_equal(self.repo.ptr, self.ptr, other.ptr) }
    }
}

impl<'a, T: Contents> Tree<'a, T> {
    /// Create an empty tree
    pub fn new(repo: &'a Repo<T>) -> Result<Tree<'a, T>, Error> {
        unsafe {
            let ptr = irmin_tree_new(repo.ptr);
            check!(repo.ptr, ptr);
            Ok(Tree {
                ptr,
                repo: UntypedRepo::new(repo),
                _t: std::marker::PhantomData,
            })
        }
    }

    /// Compute the hash of a tree
    pub fn hash(&self) -> Result<Hash, Error> {
        let h = unsafe { irmin_tree_hash(self.repo.ptr, self.ptr) };
        check!(self.repo.ptr, h);
        Ok(Hash {
            ptr: h,
            repo: self.repo.clone(),
        })
    }

    ///
    pub fn key(&self) -> Result<Option<KindedKey>, Error> {
        let h = unsafe { irmin_tree_key(self.repo.ptr, self.ptr) };
        check_opt!(self.repo.ptr, h);
        Ok(Some(KindedKey {
            ptr: h,
            repo: self.repo.clone(),
        }))
    }

    pub fn of_hash(repo: &'a Repo<T>, h: &Hash) -> Result<Option<Tree<'a, T>>, Error> {
        let ptr = unsafe { irmin_tree_of_hash(repo.ptr, h.ptr) };
        check_opt!(repo.ptr, ptr);
        Ok(Some(Tree {
            ptr,
            repo: UntypedRepo::new(repo),
            _t: std::marker::PhantomData,
        }))
    }

    pub fn of_key(repo: &'a Repo<T>, k: &KindedKey) -> Result<Option<Tree<'a, T>>, Error> {
        let ptr = unsafe { irmin_tree_of_key(repo.ptr, k.ptr) };
        check_opt!(repo.ptr, ptr);
        Ok(Some(Tree {
            ptr,
            repo: UntypedRepo::new(repo),
            _t: std::marker::PhantomData,
        }))
    }

    /// Update the tree with a value at the specified path
    pub fn add(
        &mut self,
        path: &Path,
        value: &T,
        metadata: Option<&Metadata>,
    ) -> Result<(), Error> {
        let x = unsafe {
            let value = value.to_value()?;
            let meta = match metadata {
                Some(m) => m.ptr,
                None => std::ptr::null_mut(),
            };
            irmin_tree_add(self.repo.ptr, self.ptr, path.ptr, value.ptr as *mut _, meta)
        };
        check!(self.repo.ptr, x, false);
        Ok(())
    }

    /// Update the tree with a tree at the specified path
    pub fn add_tree(&mut self, path: &Path, tree: &Tree<T>) -> Result<(), Error> {
        let x = unsafe { irmin_tree_add_tree(self.repo.ptr, self.ptr, path.ptr, tree.ptr) };
        check!(self.repo.ptr, x, false);
        Ok(())
    }

    /// Check for the existence of a value at the given path
    pub fn mem(&self, path: &Path) -> bool {
        unsafe { irmin_tree_mem(self.repo.ptr, self.ptr, path.ptr) }
    }

    /// Check for the existence of a tree at the given path
    pub fn mem_tree(&self, path: &Path) -> bool {
        unsafe { irmin_tree_mem_tree(self.repo.ptr, self.ptr, path.ptr) }
    }

    /// Remove any bindings for the given path
    pub fn remove(&mut self, path: &Path) -> Result<(), Error> {
        let x = unsafe { irmin_tree_remove(self.repo.ptr, self.ptr, path.ptr) };
        check!(self.repo.ptr, x, false);
        Ok(())
    }

    /// Find a value associated with a path
    pub fn find(&self, path: &Path) -> Result<Option<T>, Error> {
        unsafe {
            let ptr = irmin_tree_find(self.repo.ptr, self.ptr, path.ptr);
            check!(self.repo.ptr, ptr);
            if ptr.is_null() {
                return Ok(None);
            }
            let ty = T::ty()?;
            let x = Value {
                ptr: ptr as *mut _,
                ty,
            };
            let value = T::from_value(&x)?;
            Ok(Some(value))
        }
    }

    /// Find a tree associated with a path
    pub fn find_tree(&self, path: &Path) -> Result<Option<Tree<T>>, Error> {
        unsafe {
            let ptr = irmin_tree_find_tree(self.repo.ptr, self.ptr, path.ptr);
            check_opt!(self.repo.ptr, ptr);
            let x = Tree {
                ptr,
                repo: self.repo.clone(),
                _t: std::marker::PhantomData,
            };
            Ok(Some(x))
        }
    }

    /// List paths
    pub fn list(&self, path: &Path) -> Result<Vec<Path>, Error> {
        let p = unsafe { irmin_tree_list(self.repo.ptr, self.ptr, path.ptr) };
        check!(self.repo.ptr, p);
        let len = unsafe { irmin_path_array_length(self.repo.ptr, p) };
        let mut dest = Vec::new();
        for i in 0..len {
            let path = unsafe { irmin_path_array_get(self.repo.ptr, p, i) };
            if path.is_null() {
                continue;
            }
            dest.push(Path {
                ptr: path,
                repo: self.repo.clone(),
            })
        }

        unsafe { irmin_path_array_free(p) }

        Ok(dest)
    }
}
