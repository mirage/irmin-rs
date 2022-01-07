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
        check!(ptr);
        Ok(Type { ptr })
    }

    /// Irmin.Type.int
    pub fn int() -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_int() };
        check!(ptr);
        Ok(Type { ptr })
    }

    /// Irmin.Type.float
    pub fn float() -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_float() };
        check!(ptr);
        Ok(Type { ptr })
    }

    /// Irmin.Type.bool
    pub fn bool() -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_bool() };
        check!(ptr);
        Ok(Type { ptr })
    }

    /// Irmin.Contents.Json.t
    pub fn json() -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_json() };
        check!(ptr);
        Ok(Type { ptr })
    }

    /// Irmin.Contents.Json_value.t
    pub fn json_value() -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_json_value() };
        check!(ptr);
        Ok(Type { ptr })
    }

    /// The path type for a Repo
    pub fn path<T: Contents>(repo: &Repo<T>) -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_path(repo.ptr) };
        check!(ptr);
        Ok(Type { ptr })
    }

    /// The hash type for a Repo
    pub fn hash<T: Contents>(repo: &Repo<T>) -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_hash(repo.ptr) };
        check!(ptr);
        Ok(Type { ptr })
    }

    /// The commit type for a Repo
    pub fn commit<T: Contents>(repo: &Repo<T>) -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_commit(repo.ptr) };
        check!(ptr);
        Ok(Type { ptr })
    }

    /// The tree type for a Repo
    pub fn tree<T: Contents>(repo: &Repo<T>) -> Result<Type, Error> {
        let ptr = unsafe { irmin_type_tree(repo.ptr) };
        check!(ptr);
        Ok(Type { ptr })
    }

    /// Get the name of a type
    pub fn name(&self) -> Result<IrminString, Error> {
        let name = unsafe { irmin_type_name(self.ptr) };
        check!(name);
        IrminString::wrap(name)
    }
}
