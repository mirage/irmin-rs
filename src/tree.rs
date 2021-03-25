use crate::Type;

pub enum Tree<T: Type> {
    Contents(T),
}
