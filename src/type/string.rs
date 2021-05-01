#[derive(Debug, PartialEq, PartialOrd)]
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
