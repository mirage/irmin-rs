use crate::internal::*;

/// Wrapper around Irmin.config
pub struct Config<T: Contents> {
    pub ptr: *mut IrminConfig,
    _t: std::marker::PhantomData<T>,
}

impl<T: Contents> Drop for Config<T> {
    fn drop(&mut self) {
        unsafe { irmin_config_free(self.ptr) }
    }
}

/// Builtin content types
pub enum ContentType {
    String,
    Json,
    JsonValue,
}

const CONTENTS_STRING: &str = "string\0";

const CONTENTS_JSON: &str = "json\0";

const CONTENTS_JSON_VALUE: &str = "json-value\0";

/// Used to specify the content type of a store
pub trait Contents
where
    Self: Sized,
{
    fn content_type() -> ContentType;
    fn to_value(&self) -> Result<Value, Error>;
    fn from_value(v: &Value) -> Result<Self, Error>;

    fn of_hash<'a>(repo: &'a Repo<Self>, hash: &Hash) -> Option<Self> {
        let ptr = unsafe { irmin_contents_of_hash(repo.ptr, hash.ptr) };
        if ptr.is_null() {
            return None;
        }
        let ty = match Type::contents(repo) {
            Ok(t) => t,
            Err(_) => return None,
        };
        let v = Value {
            ptr: ptr as *mut _,
            ty,
        };
        match Self::from_value(&v) {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }

    fn hash<'a>(&self, repo: &'a Repo<Self>) -> Result<Hash<'a>, Error> {
        let v = self.to_value()?;

        let ptr = unsafe { irmin_contents_hash(repo.ptr, v.ptr as *mut _) };
        check!(repo.ptr, ptr);
        Ok(Hash {
            ptr,
            repo: UntypedRepo::new(repo),
        })
    }

    fn ty() -> Result<Type, Error> {
        match Self::content_type() {
            ContentType::String => Type::string(),
            ContentType::Json => Type::json(),
            ContentType::JsonValue => Type::json_value(),
        }
    }
}

impl ContentType {
    fn ptr(c: Option<ContentType>) -> *const u8 {
        match c {
            Some(ContentType::String) => CONTENTS_STRING.as_ptr(),
            Some(ContentType::Json) => CONTENTS_JSON.as_ptr(),
            Some(ContentType::JsonValue) => CONTENTS_JSON_VALUE.as_ptr(),
            None => std::ptr::null(),
        }
    }
}

/// Available hash types
pub enum HashType {
    Blake2b,
    Blake2s,
    Rmd160,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

const HASH_SHA1: &str = "sha1\0";
const HASH_SHA224: &str = "sha224\0";
const HASH_SHA256: &str = "sha256\0";
const HASH_SHA384: &str = "sha384\0";
const HASH_SHA512: &str = "sha512\0";
const HASH_RMD160: &str = "rmd160\0";

const HASH_BLAKE2B: &str = "blake2b\0";

const HASH_BLAKE2S: &str = "blake2s\0";

impl HashType {
    fn ptr(h: Option<HashType>) -> *const u8 {
        match h {
            Some(HashType::Sha1) => HASH_SHA1.as_ptr(),
            Some(HashType::Sha224) => HASH_SHA224.as_ptr(),
            Some(HashType::Sha256) => HASH_SHA256.as_ptr(),
            Some(HashType::Sha384) => HASH_SHA384.as_ptr(),
            Some(HashType::Sha512) => HASH_SHA512.as_ptr(),
            Some(HashType::Blake2b) => HASH_BLAKE2B.as_ptr(),
            Some(HashType::Blake2s) => HASH_BLAKE2S.as_ptr(),
            Some(HashType::Rmd160) => HASH_RMD160.as_ptr(),
            None => std::ptr::null(),
        }
    }
}

impl Contents for IrminString {
    fn content_type() -> ContentType {
        ContentType::String
    }

    fn to_value(&self) -> Result<Value, Error> {
        Value::string(self)
    }

    fn from_value(v: &Value) -> Result<Self, Error> {
        v.get_string()
    }
}

impl Contents for String {
    fn content_type() -> ContentType {
        ContentType::String
    }

    fn to_value(&self) -> Result<Value, Error> {
        Value::string(self)
    }

    fn from_value(v: &Value) -> Result<Self, Error> {
        v.get_string().map(|x| x.into())
    }
}

impl Contents for Vec<u8> {
    fn content_type() -> ContentType {
        ContentType::String
    }

    fn to_value(&self) -> Result<Value, Error> {
        Value::bytes(self)
    }

    fn from_value(v: &Value) -> Result<Self, Error> {
        v.get_string().map(|x| x.into())
    }
}

impl Contents for serde_json::Value {
    fn content_type() -> ContentType {
        ContentType::JsonValue
    }

    fn to_value(&self) -> Result<Value, Error> {
        let ty = Type::json_value()?;
        let s = serde_json::to_string(self)?;
        Value::of_string(ty, s)
    }

    fn from_value(v: &Value) -> Result<Self, Error> {
        let s = v.to_string()?;
        serde_json::from_str(s.as_ref()).map_err(Error::from)
    }
}

impl Contents for serde_json::Map<String, serde_json::Value> {
    fn content_type() -> ContentType {
        ContentType::Json
    }

    fn to_value(&self) -> Result<Value, Error> {
        let ty = Type::json()?;
        let s = serde_json::to_string(self)?;
        Value::of_string(ty, s)
    }

    fn from_value(v: &Value) -> Result<Self, Error> {
        let s = v.to_string()?;

        serde_json::from_str(s.as_ref()).map_err(Error::from)
    }
}

impl Config<IrminString> {
    /// Create configuration for Tezos context store
    pub fn tezos() -> Result<Config<IrminString>, Error> {
        unsafe {
            let ptr = irmin_config_tezos();
            if ptr.is_null() {
                return Err(Error::NullPtr);
            }
            Ok(Config {
                ptr,
                _t: std::marker::PhantomData,
            })
        }
    }
}

impl<T: Contents> Config<T> {
    /// Create configuration for Irmin_pack store
    pub fn pack(hash: Option<HashType>) -> Result<Config<T>, Error> {
        unsafe {
            let hash = HashType::ptr(hash);
            let contents = ContentType::ptr(Some(T::content_type()));
            let ptr = irmin_config_pack(hash as *mut _, contents as *mut _);
            if ptr.is_null() {
                return Err(Error::NullPtr);
            }
            Ok(Config {
                ptr,
                _t: std::marker::PhantomData,
            })
        }
    }

    /// Create configuration for Irmin_mem store
    pub fn mem(hash: Option<HashType>) -> Result<Config<T>, Error> {
        unsafe {
            let hash = HashType::ptr(hash);
            let contents = ContentType::ptr(Some(T::content_type()));
            let ptr = irmin_config_mem(hash as *mut _, contents as *mut _);
            if ptr.is_null() {
                return Err(Error::NullPtr);
            }
            Ok(Config {
                ptr,
                _t: std::marker::PhantomData,
            })
        }
    }

    /// Create configuration for Irmin_fs store
    pub fn fs(hash: Option<HashType>) -> Result<Config<T>, Error> {
        unsafe {
            let hash = HashType::ptr(hash);
            let contents = ContentType::ptr(Some(T::content_type()));
            let ptr = irmin_config_fs(hash as *mut _, contents as *mut _);
            if ptr.is_null() {
                return Err(Error::NullPtr);
            }
            Ok(Config {
                ptr,
                _t: std::marker::PhantomData,
            })
        }
    }

    /// Create configuration for Irmin_git on-disk store
    pub fn git() -> Result<Config<T>, Error> {
        unsafe {
            let contents = ContentType::ptr(Some(T::content_type()));
            let ptr = irmin_config_git(contents as *mut _);
            if ptr.is_null() {
                return Err(Error::NullPtr);
            }
            Ok(Config {
                ptr,
                _t: std::marker::PhantomData,
            })
        }
    }

    /// Create configuration for Irmin_git in-memory store
    pub fn git_mem() -> Result<Config<T>, Error> {
        unsafe {
            let contents = ContentType::ptr(Some(T::content_type()));
            let ptr = irmin_config_git_mem(contents as *mut _);
            if ptr.is_null() {
                return Err(Error::NullPtr);
            }
            Ok(Config {
                ptr,
                _t: std::marker::PhantomData,
            })
        }
    }

    /// Set configuration key
    pub fn set(&mut self, key: impl AsRef<str>, ty: &Type, v: &Value) -> bool {
        let key = cstring(key);
        unsafe { irmin_config_set(self.ptr, key.as_ptr() as *mut _, ty.ptr, v.ptr) }
    }

    /// Set root key
    pub fn set_root(&mut self, root: impl AsRef<std::path::Path>) -> bool {
        let v = cstring(root.as_ref().to_str().expect("Invalid path"));
        unsafe { irmin_config_set_root(self.ptr, v.as_ptr() as *mut _) }
    }
}
