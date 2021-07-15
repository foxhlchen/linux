// SPDX-License-Identifier: GPL-2.0

//! Inode.

use crate::bindings;
use crate::error::*;
use core::ptr;

pub struct Inode {
    c_inode: *mut bindings::inode,
}

impl Inode {
    pub fn default() -> Inode {
        Inode {
            c_inode: ptr::null_mut(),
        }
    }

    pub fn from_c_inode(c_inode: *mut bindings::inode) -> Result<Self> {
        if c_inode.is_null() {
            return Err(Error::EINVAL);
        }

        //TODO inc refcnt, and dec in dtor
        let mut i = Inode::default();
        i.c_inode = c_inode;

        Ok(i)
    }

    pub fn to_c_inode(&self) -> *mut bindings::inode {
        self.c_inode
    }
}
