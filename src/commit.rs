use crate::internal::*;

/// Wrapper around Irmin commits
pub struct Commit<'a> {
    pub ptr: *mut IrminCommit,
    pub(crate) repo: UntypedRepo<'a>,
}

impl<'a> Drop for Commit<'a> {
    fn drop(&mut self) {
        unsafe { irmin_commit_free(self.ptr) }
    }
}

impl<'a> PartialEq for Commit<'a> {
    fn eq(&self, other: &Commit<'a>) -> bool {
        unsafe { irmin_commit_equal(self.repo.ptr, self.ptr, other.ptr) }
    }
}

impl<'a> Commit<'a> {
    /// Create a new commit
    pub fn new<T: Contents>(
        repo: &'a Repo<T>,
        parents: impl AsRef<[&'a Commit<'a>]>,
        tree: &Tree<T>,
        info: Info,
    ) -> Result<Commit<'a>, Error> {
        let parents: Vec<_> = parents.as_ref().into_iter().map(|x| x.ptr).collect();
        let ptr = unsafe {
            irmin_commit_new(
                repo.ptr,
                parents.as_ptr() as *mut _,
                parents.len() as u64,
                tree.ptr,
                info.ptr,
            )
        };
        check!(ptr);
        Ok(Commit {
            ptr,
            repo: UntypedRepo::new(repo),
        })
    }

    /// Find the commit associated with the given hash
    pub fn of_hash<T: Contents>(repo: &'a Repo<T>, hash: &Hash) -> Option<Commit<'a>> {
        let ptr = unsafe { irmin_commit_of_hash(repo.ptr, hash.ptr) };
        if ptr.is_null() {
            return None;
        }
        Some(Commit {
            ptr,
            repo: UntypedRepo::new(repo),
        })
    }

    /// Get the hash associated with a commit
    pub fn hash(&self) -> Result<Hash, Error> {
        let ptr = unsafe { irmin_commit_hash(self.repo.ptr, self.ptr) };
        check!(ptr);
        Ok(Hash {
            ptr,
            repo: self.repo.clone(),
        })
    }

    /// Find the commit associated with the given key
    pub fn of_key<T: Contents>(repo: &'a Repo<T>, key: &CommitKey) -> Option<Commit<'a>> {
        let ptr = unsafe { irmin_commit_of_key(repo.ptr, key.ptr) };
        if ptr.is_null() {
            return None;
        }
        Some(Commit {
            ptr,
            repo: UntypedRepo::new(repo),
        })
    }

    /// Get the key associated with a commit
    pub fn key(&self) -> Result<CommitKey, Error> {
        let ptr = unsafe { irmin_commit_key(self.repo.ptr, self.ptr) };
        check!(ptr);
        Ok(CommitKey {
            ptr,
            repo: self.repo.clone(),
        })
    }

    /// Get commit info
    pub fn info(&self) -> Result<Info, Error> {
        let ptr = unsafe { irmin_commit_info(self.repo.ptr, self.ptr) };
        check!(ptr);
        Ok(Info {
            ptr,
            repo: self.repo.clone(),
        })
    }

    /// Get commit parents
    pub fn parents(&self) -> Result<Vec<Commit>, Error> {
        let p = unsafe { irmin_commit_parents(self.repo.ptr, self.ptr) };
        check!(p);
        let len = unsafe { irmin_commit_list_length(self.repo.ptr, p) };
        let mut dest = Vec::new();
        for i in 0..len {
            let c = unsafe { irmin_commit_list_get(self.repo.ptr, p, i) };
            if c.is_null() {
                continue;
            }
            dest.push(Commit {
                ptr: c,
                repo: self.repo.clone(),
            })
        }

        unsafe { irmin_commit_list_free(p) }

        Ok(dest)
    }
}
