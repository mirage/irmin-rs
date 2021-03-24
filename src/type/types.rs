pub type Int = isize;

pub type Pair<T, U> = (T, U);
pub type Triple<T, U, V> = (T, U, V);

pub enum Array<'a, T> {
    Owned(Vec<T>),
    Ref(&'a [T]),
}

impl<'a, T> Array<'a, T> {
    pub fn is_empty(&self) -> bool {
        match self {
            Array::Owned(a) => a.is_empty(),
            Array::Ref(a) => a.is_empty(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Array::Owned(a) => a.len(),
            Array::Ref(a) => a.len(),
        }
    }
}

impl<'a, T> AsRef<[T]> for Array<'a, T> {
    fn as_ref(&self) -> &[T] {
        match self {
            Array::Owned(x) => x.as_ref(),
            Array::Ref(x) => x,
        }
    }
}

impl<'a, T> From<&'a [T]> for Array<'a, T> {
    fn from(x: &'a [T]) -> Self {
        Array::Ref(x)
    }
}

impl<'a, T> From<Vec<T>> for Array<'a, T> {
    fn from(x: Vec<T>) -> Self {
        Array::Owned(x)
    }
}

pub enum Str<'a> {
    Owned(String),
    Ref(&'a str),
}

impl<'a> Str<'a> {
    pub fn is_empty(&self) -> bool {
        match self {
            Str::Owned(a) => a.is_empty(),
            Str::Ref(a) => a.is_empty(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Str::Owned(a) => a.len(),
            Str::Ref(a) => a.len(),
        }
    }
}

impl<'a> AsRef<str> for Str<'a> {
    fn as_ref(&self) -> &str {
        match self {
            Str::Owned(x) => x.as_ref(),
            Str::Ref(x) => x,
        }
    }
}

impl<'a> From<&'a str> for Str<'a> {
    fn from(x: &'a str) -> Self {
        Str::Ref(x)
    }
}

impl<'a> From<String> for Str<'a> {
    fn from(x: String) -> Self {
        Str::Owned(x)
    }
}

pub enum Bytes<'a> {
    Owned(Vec<u8>),
    Ref(&'a [u8]),
}

impl<'a> AsRef<[u8]> for Bytes<'a> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Bytes::Owned(x) => x.as_ref(),
            Bytes::Ref(x) => x,
        }
    }
}

impl<'a> From<&'a [u8]> for Bytes<'a> {
    fn from(x: &'a [u8]) -> Self {
        Bytes::Ref(x)
    }
}

impl<'a> From<Vec<u8>> for Bytes<'a> {
    fn from(x: Vec<u8>) -> Self {
        Bytes::Owned(x)
    }
}
