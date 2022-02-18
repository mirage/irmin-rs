#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_export]
macro_rules! check {
    ($r:expr, $x:expr) => {
        if $x.is_null() {
            match crate::error_msg($r) {
                Some(e) => return Err(Error::Exc(e)),
                None => return Err(Error::NullPtr),
            }
        }
    };
    ($r:expr, $x:expr, $y:expr) => {
        if $x == $y {
            match crate::error_msg($r) {
                Some(e) => return Err(Error::Exc(e)),
                None => (),
            }
        }
    };
}

#[macro_export]
macro_rules! check_opt {
    ($r:expr, $x:expr) => {
        if $x.is_null() {
            match crate::error_msg($r) {
                Some(e) => return Err(Error::Exc(e)),
                None => return Ok(None),
            }
        }
    };
}

include!(concat!(env!("OUT_DIR"), "/c.rs"));
