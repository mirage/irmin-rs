use crate::internal::*;

/// Wrapper around Irmin.Type
pub struct Type {
    pub ptr: *mut IrminType,
}

impl Clone for Type {
    fn clone(&self) -> Type {
        let ptr = unsafe { irmin_value_clone(self.ptr as *mut _) as *mut IrminType };
        Type { ptr }
    }
}

impl Drop for Type {
    fn drop(&mut self) {
        unsafe { irmin_type_free(self.ptr) }
    }
}

impl Type {
    /// Irmin.Type.string
    pub fn string() -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_string() };
        if ptr.is_null() {
            return Err(Error::NullPtr);
        };
        Ok(Type { ptr })
    }

    /// Irmin.Type.int
    pub fn int() -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_int() };
        if ptr.is_null() {
            return Err(Error::NullPtr);
        };
        Ok(Type { ptr })
    }

    /// Irmin.Type.float
    pub fn float() -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_float() };
        if ptr.is_null() {
            return Err(Error::NullPtr);
        };
        Ok(Type { ptr })
    }

    /// Irmin.Type.bool
    pub fn bool() -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_bool() };
        if ptr.is_null() {
            return Err(Error::NullPtr);
        };
        Ok(Type { ptr })
    }

    /// Irmin.Contents.Json.t
    pub fn json() -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_json() };
        if ptr.is_null() {
            return Err(Error::NullPtr);
        };
        Ok(Type { ptr })
    }

    /// Irmin.Contents.Json_value.t
    pub fn json_value() -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_json_value() };
        if ptr.is_null() {
            return Err(Error::NullPtr);
        };
        Ok(Type { ptr })
    }

    /// The path type for a Repo
    pub fn path<T: Contents>(repo: &Repo<T>) -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_path(repo.ptr) };
        check!(repo.ptr, ptr);
        Ok(Type { ptr })
    }

    /// The hash type for a Repo
    pub fn hash<T: Contents>(repo: &Repo<T>) -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_hash(repo.ptr) };
        check!(repo.ptr, ptr);
        Ok(Type { ptr })
    }

    /// The commit type for a Repo
    pub fn commit<T: Contents>(repo: &Repo<T>) -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_commit(repo.ptr) };
        check!(repo.ptr, ptr);
        Ok(Type { ptr })
    }

    pub fn commit_key<T: Contents>(repo: &Repo<T>) -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_commit_key(repo.ptr) };
        check!(repo.ptr, ptr);
        Ok(Type { ptr })
    }

    pub fn kinded_key<T: Contents>(repo: &Repo<T>) -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_kinded_key(repo.ptr) };
        check!(repo.ptr, ptr);
        Ok(Type { ptr })
    }

    pub fn metadata<T: Contents>(repo: &Repo<T>) -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_metadata(repo.ptr) };
        check!(repo.ptr, ptr);
        Ok(Type { ptr })
    }

    pub fn contents<T: Contents>(repo: &Repo<T>) -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_contents(repo.ptr) };
        check!(repo.ptr, ptr);
        Ok(Type { ptr })
    }

    /// The tree type for a Repo
    pub fn tree<T: Contents>(repo: &Repo<T>) -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_tree(repo.ptr) };
        check!(repo.ptr, ptr);
        Ok(Type { ptr })
    }

    /// Get the name of a type
    pub fn name(&self) -> Result<IrminString, Error> {
        let name = unsafe { irmin_type_name(self.ptr) };
        IrminString::wrap(name)
    }
}
