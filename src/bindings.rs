#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_export]
macro_rules! check {
    ($x:expr) => {
        if $x.is_null() {
            match crate::error_msg() {
                Some(e) => return Err(Error::Exc(e)),
                None => return Err(Error::NullPtr),
            }
        }
    };
    ($x:expr, $y:expr) => {
        if $x == $y {
            match crate::error_msg() {
                Some(e) => return Err(Error::Exc(e)),
                None => (),
            }
        }
    };
}

include!(concat!(env!("OUT_DIR"), "/c.rs"));
