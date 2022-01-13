use crate::internal::*;

/// Wrapper around OCaml values
pub struct Value {
    pub ty: Type,
    pub ptr: *mut IrminValue,
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe { irmin_value_free(self.ptr) }
    }
}

impl Clone for Value {
    fn clone(&self) -> Value {
        let ptr = unsafe { irmin_value_clone(self.ptr as *mut _) };
        Value {
            ty: self.ty.clone(),
            ptr,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        let x = match (self.ty.name(), other.ty.name()) {
            (Ok(a), Ok(b)) => a == b,
            (_, _) => return false,
        };
        x && unsafe { irmin_value_equal(self.ty.ptr, self.ptr, other.ptr) }
    }
}

impl Value {
    /// OCaml string
    pub fn string(s: impl AsRef<str>) -> Result<Value, Error> {
        let s = s.as_ref();
        let ptr =
            unsafe { irmin_string_new(s.as_ptr() as *mut _, s.len() as i64) as *mut IrminValue };
        check!(ptr);

        let ty = Type::string()?;

        Ok(Value { ptr, ty })
    }

    /// OCaml string from Rust &[u8]
    pub fn bytes(s: impl AsRef<[u8]>) -> Result<Value, Error> {
        let s = s.as_ref();
        let ptr =
            unsafe { irmin_string_new(s.as_ptr() as *mut _, s.len() as i64) as *mut IrminValue };
        check!(ptr);

        let ty = Type::string()?;

        Ok(Value { ptr, ty })
    }

    /// OCaml int
    pub fn int(i: i64) -> Result<Value, Error> {
        let ptr = unsafe { irmin_value_int(i) };
        check!(ptr);

        let ty = Type::int()?;

        Ok(Value { ptr, ty })
    }

    /// OCaml float
    pub fn float(i: f64) -> Result<Value, Error> {
        let ptr = unsafe { irmin_value_float(i) };
        check!(ptr);

        let ty = Type::float()?;

        Ok(Value { ptr, ty })
    }

    /// OCaml bool
    pub fn bool(i: bool) -> Result<Value, Error> {
        let ptr = unsafe { irmin_value_bool(i) };
        check!(ptr);

        let ty = Type::bool()?;

        Ok(Value { ptr, ty })
    }

    /// Parse a value of the specified type from Irmin's string encoding
    pub fn of_string(ty: Type, s: impl AsRef<str>) -> Result<Value, Error> {
        let s = s.as_ref();

        let ptr = unsafe { irmin_value_of_string(ty.ptr, s.as_ptr() as *mut _, s.len() as i64) };
        check!(ptr);

        Ok(Value { ptr, ty })
    }

    /// Encode a value using Irmin's string encoding
    pub fn to_string(&self) -> Result<IrminString, Error> {
        let s = unsafe { irmin_value_to_string(self.ty.ptr, self.ptr) };
        crate::IrminString::wrap(s)
    }

    /// Parse a value of the specified type from Irmin's JSON encoding
    pub fn of_json(ty: Type, s: impl AsRef<str>) -> Result<Value, Error> {
        let s = s.as_ref();

        let ptr = unsafe { irmin_value_of_json(ty.ptr, s.as_ptr() as *mut _, s.len() as i64) };
        check!(ptr);
        Ok(Value { ptr, ty })
    }

    /// Encode a value using Irmin's JSON encoding
    pub fn to_json(&self) -> Result<IrminString, Error> {
        let s = unsafe { irmin_value_to_json(self.ty.ptr, self.ptr) };
        crate::IrminString::wrap(s)
    }

    /// Parse a value of the specified type from Irmin's binary encoding
    pub fn of_bin(ty: Type, s: impl AsRef<[u8]>) -> Result<Value, Error> {
        let s = s.as_ref();
        let ptr = unsafe { irmin_value_of_bin(ty.ptr, s.as_ptr() as *mut _, s.len() as i64) };
        check!(ptr);
        Ok(Value { ptr, ty })
    }

    /// Encode a value using Irmin's binary encoding
    pub fn to_bin(&self) -> Result<IrminString, Error> {
        let s = unsafe { irmin_value_to_bin(self.ty.ptr, self.ptr) };
        crate::IrminString::wrap(s)
    }

    /// Get IrminString from string value
    pub fn get_string(&self) -> Result<IrminString, Error> {
        let s = unsafe { irmin_value_get_string(self.ptr) };
        crate::IrminString::wrap(s)
    }
}
