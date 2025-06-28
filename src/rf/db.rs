use crate::{impl_ops_via_intermediary, Number};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Db(pub Number);

impl Into<Number> for Db {
    fn into(self) -> Number {
        self.0
    }
}
impl Into<Number> for &Db {
    fn into(self) -> Number {
        self.0
    }
}
impl From<Number> for Db {
    fn from(value: Number) -> Self {
        Self(value)
    }
}

impl_ops_via_intermediary!(Db, Number);
