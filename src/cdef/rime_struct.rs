use super::{RimeCommit, RimeContext, RimeStatus, RimeTraits};

use std::mem;
use std::os::raw::c_int;

/// Many structs used in rime has data_size as the first field,
/// which is the size of the struct except the data_size field itself.
/// Rime uses this to achieve backward-compatibility.
/// Users of the API doesn't need to define all fields, rime will only
/// read the fields within the bound of data_size
pub trait RimeSizedStruct {
    unsafe fn uninitialized() -> Self;
}

impl<T: SetDataSize> RimeSizedStruct for T {
    unsafe fn uninitialized() -> Self {
        let mut var: T;
        var = mem::uninitialized();
        var.set_data_size(data_size_of::<T>());
        var
    }
}

/// For structs with a data_size field
pub trait SetDataSize {
    fn set_data_size(&mut self, size: c_int);
}

fn data_size_of<T>() -> c_int {
    let size = mem::size_of::<T>() - mem::size_of::<c_int>();
    size as c_int
}

impl SetDataSize for RimeCommit {
    fn set_data_size(&mut self, size: i32) {
        self.data_size = size
    }
}

impl SetDataSize for RimeContext {
    fn set_data_size(&mut self, size: i32) {
        self.data_size = size
    }
}

impl SetDataSize for RimeStatus {
    fn set_data_size(&mut self, size: i32) {
        self.data_size = size
    }
}

impl SetDataSize for RimeTraits {
    fn set_data_size(&mut self, size: i32) {
        self.data_size = size
    }
}
