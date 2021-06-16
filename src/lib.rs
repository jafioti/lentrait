use std::{collections::VecDeque};

pub trait Len {
    fn len(&self) -> usize;
}

// IMPLEMENTATIONS
impl Len for str {
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len for String {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for VecDeque<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: ?Sized + Len> Len for Box<T> {
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}

impl<T> Len for [T] {
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len for pyo3::PyAny {
    fn len(&self) -> usize {
        self.len().expect("Failed to get length!")
    }
}

impl Len for &pyo3::PyAny {
    fn len(&self) -> usize {
        (*self).len().expect("Failed to get length!")
    }
}

impl Len for serde_json::Value {
    fn len(&self) -> usize {
        self.to_owned().len()
    }
}

impl Len for &serde_json::Value {
    fn len(&self) -> usize {
        (*self).len()
    }
}

impl Len for mako::vocab::Vocab {
    fn len(&self) -> usize {
        self.num_tokens as usize
    }
}