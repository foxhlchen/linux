// SPDX-License-Identifier: GPL-2.0

//! Dentry.

use crate::bindings;
use crate::error::*;
use core::ptr;

pub struct Dentry {
    c_dentry: *mut bindings::dentry,
}

impl Dentry {
    pub fn default() -> Dentry {
        Dentry {
            c_dentry: ptr::null_mut(),
        }
    }

    pub fn from_c_dentry(c_dentry: *mut bindings::dentry) -> Result<Self> {
        if c_dentry.is_null() {
            return Err(Error::EINVAL);
        }

        //TODO inc refcnt, and dec in dtor
        let mut d = Dentry::default();
        d.c_dentry = c_dentry;

        Ok(d)
    }

    pub fn to_c_dentry(&self) -> *mut bindings::dentry {
        self.c_dentry
    }
}
