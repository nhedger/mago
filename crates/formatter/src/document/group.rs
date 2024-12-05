use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use core::ops::AddAssign;

use serde::Serialize;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, PartialOrd, Ord)]
pub struct GroupIdentifier(pub usize);

#[derive(Debug)]
pub struct GroupIdentifierBuilder {
    id: GroupIdentifier,
}

impl GroupIdentifierBuilder {
    pub fn new() -> Self {
        Self { id: GroupIdentifier(0) }
    }

    pub fn next_id(&mut self) -> GroupIdentifier {
        self.id += 1;
        self.id
    }
}

impl AddAssign<usize> for GroupIdentifier {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl Display for GroupIdentifier {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "group-identifier({})", self.0)
    }
}