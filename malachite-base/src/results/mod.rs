use std::fmt::{Debug, Display};
use strings::{ExtraToString, ToDebugString};

//TODO doc and test

impl<T: Display, E: Debug> ExtraToString for Result<T, E> {
    fn to_string(&self) -> String {
        let mut s = String::new();
        match self {
            Ok(x) => {
                s.push_str("Ok(");
                s.push_str(&x.to_string());
                s.push(')');
            }
            Err(e) => {
                s.push_str("Err(");
                s.push_str(&e.to_debug_string());
                s.push(')');
            }
        }
        s
    }
}
