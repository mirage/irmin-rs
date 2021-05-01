#[derive(Debug, PartialEq, PartialOrd)]
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
