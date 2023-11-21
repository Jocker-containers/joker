use std::{error, fmt};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct AbsentHashMapKeyError;

impl Display for AbsentHashMapKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No such key in a hashmap.")
    }
}

impl error::Error for AbsentHashMapKeyError {}
