// SPDX-License-Identifier: GPL-2.0

//! UserNameSpace.

use crate::bindings;
use crate::error::*;
use core::ptr;

pub struct UserNameSpace {
    c_user_ns: *mut bindings::user_namespace,
}

impl UserNameSpace {
    pub fn default() -> UserNameSpace {
        UserNameSpace {
            c_user_ns: ptr::null_mut(),
        }
    }

    pub fn from_c_user_namespace(c_user_ns: *mut bindings::user_namespace) -> Result<Self> {
        if c_user_ns.is_null() {
            return Err(Error::EINVAL);
        }

        let mut ns = Self::default();
        ns.c_user_ns = c_user_ns;

        Ok(ns)
    }

    pub fn to_c_user_namespace(&self) -> *mut bindings::user_namespace {
        self.c_user_ns
    }
}
