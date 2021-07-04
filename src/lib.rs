use std::{collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque}, ffi::{CStr, CString, OsStr, OsString}, hash::Hash};

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

impl<K: Eq + Hash, V> Len for HashMap<K, V> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<K: Eq + Hash> Len for HashSet<K> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for LinkedList<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len for CStr {
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}

impl Len for CString {
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}

impl Len for OsStr {
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len for OsString {
    fn len(&self) -> usize {
        self.as_os_str().len()
    }
}


impl<K: Ord, V> Len for BTreeMap<K, V> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: Ord> Len for BTreeSet<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: Ord> Len for BinaryHeap<T> {
    fn len(&self) -> usize {
        self.len()
    }
}