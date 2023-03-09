use crate_b::Inner;

pub struct Outer {
    pub inner: Inner,
}

impl Outer {
    pub fn new() -> Self {
        Outer {
            inner: Inner { field: 0 },
        }
    }
}