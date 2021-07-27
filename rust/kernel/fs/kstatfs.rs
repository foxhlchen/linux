// SPDX-License-Identifier: GPL-2.0

//! KStatfs.

use crate::bindings;
use crate::error::*;
use core::ptr;

pub struct KStatFs {
    c_kstatfs: *mut bindings::kstatfs,
}

impl KStatFs {
    pub fn default() -> KStatFs {
        KStatFs {
            c_kstatfs: ptr::null_mut(),
        }
    }

    pub fn from_c_kstatfs(c_kstatfs: *mut bindings::kstatfs) -> Result<Self> {
        if c_kstatfs.is_null() {
            return Err(Error::EINVAL);
        }

        let mut kstatfs = KStatFs::default();
        kstatfs.c_kstatfs = c_kstatfs;

        Ok(kstatfs)
    }

    pub fn to_c_kstatfs(&self) -> *mut bindings::kstatfs {
        self.c_kstatfs
    }
}
