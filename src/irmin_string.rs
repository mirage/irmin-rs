use crate::internal::*;

/// IrminString is a wrapper around strings returned by libirmin
pub struct IrminString(pub *mut crate::bindings::IrminString, pub usize);

impl std::fmt::Debug for IrminString {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(fmt)
    }
}

impl Drop for IrminString {
    fn drop(&mut self) {
        unsafe { irmin_string_free(self.0 as *mut _) }
    }
}

impl IrminString {
    pub(crate) fn wrap(ptr: *mut crate::bindings::IrminString) -> Result<IrminString, Error> {
        check!(ptr);
        let len = unsafe { irmin_string_length(ptr) };
        Ok(IrminString(ptr, len as usize))
    }

    pub fn new(s: impl AsRef<[u8]>) -> Result<IrminString, Error> {
        let len = s.as_ref().len();
        let s = unsafe { irmin_string_new(s.as_ref().as_ptr() as *mut _, len as i64) };
        check!(s);
        Ok(IrminString(s, len))
    }

    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }
}

impl PartialEq for IrminString {
    fn eq(&self, other: &IrminString) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl AsRef<[u8]> for IrminString {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            let data = irmin_string_data(self.0);
            std::slice::from_raw_parts_mut(data as *mut u8, self.1)
        }
    }
}

impl AsRef<str> for IrminString {
    fn as_ref(&self) -> &str {
        unsafe {
            let data = irmin_string_data(self.0);
            let s = std::slice::from_raw_parts_mut(data as *mut u8, self.1);
            std::str::from_utf8_unchecked(s)
        }
    }
}

impl AsRef<std::ffi::CStr> for IrminString {
    fn as_ref(&self) -> &std::ffi::CStr {
        unsafe {
            let data = irmin_string_data(self.0);
            let b = std::slice::from_raw_parts_mut(data as *mut u8, self.1 + 1);
            std::ffi::CStr::from_bytes_with_nul_unchecked(b)
        }
    }
}

impl Into<String> for IrminString {
    fn into(self) -> String {
        self.as_str().to_string()
    }
}

impl Into<Vec<u8>> for IrminString {
    fn into(self) -> Vec<u8> {
        self.as_slice().into()
    }
}
