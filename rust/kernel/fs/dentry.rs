// SPDX-License-Identifier: GPL-2.0

//! Dentry.

use super::c_types;
use crate::bindings;
use crate::error::*;
use crate::pr_warn;
use core::marker;
use core::ptr;

// unsafe extern "C" fn d_revalidate_callback<T: DentryOperations>(arg1: *mut dentry, arg2: c_types::c_uint) -> c_types::c_int {}
// unsafe extern "C" fn d_weak_revalidate_callback<T: DentryOperations>(arg1: *mut dentry, arg2: c_types::c_uint) -> c_types::c_int {}
// unsafe extern "C" fn d_hash_callback<T: DentryOperations>(arg1: *const dentry, arg2: *mut qstr) -> c_types::c_int {}
// unsafe extern "C" fn d_compare_callback<T: DentryOperations>(
//         arg1: *const dentry,
//         arg2: c_types::c_uint,
//         arg3: *const c_types::c_char,
//         arg4: *const qstr,
//     ) -> c_types::c_int {}
unsafe extern "C" fn d_delete_callback<T: DentryOperations>(
    c_dentry: *const bindings::dentry,
) -> c_types::c_int {
    let dentry_rs = Dentry::from_c_dentry(c_dentry as *mut _);
    if dentry_rs.is_err() {
        pr_warn!("Invalid inode in destroy_inode");
        return 1;
    }

    let dentry = dentry_rs.unwrap();
    T::d_delete(&dentry) as _
}
// unsafe extern "C" fn d_init_callback<T: DentryOperations>(arg1: *mut dentry) -> c_types::c_int{}
// unsafe extern "C" fn d_release_callback<T: DentryOperations>(arg1: *mut dentry){}
// unsafe extern "C" fn d_prune_callback<T: DentryOperations>(arg1: *mut dentry){}
// unsafe extern "C" fn d_iput_callback<T: DentryOperations>(arg1: *mut dentry, arg2: *mut inode){}
// unsafe extern "C" fn d_dname_callback<T: DentryOperations>(
//         arg1: *mut dentry,
//         arg2: *mut c_types::c_char,
//         arg3: c_types::c_int,
//     ) -> *mut c_types::c_char {}
// unsafe extern "C" fn d_automount_callback<T: DentryOperations>(arg1: *mut path) -> *mut vfsmount{}
// unsafe extern "C" fn d_manage_callback<T: DentryOperations>(arg1: *const path, arg2: bool_) -> c_types::c_int {}
// unsafe extern "C" fn d_real_callback<T: DentryOperations>(arg1: *mut dentry, arg2: *const inode) -> *mut dentry {}

pub(crate) struct DentryOperationsVtable<T>(marker::PhantomData<T>);

impl<T: DentryOperations> DentryOperationsVtable<T> {
    const VTABLE: bindings::dentry_operations = bindings::dentry_operations {
        d_revalidate: None,
        d_weak_revalidate: None,
        d_hash: None,
        d_compare: None,
        d_delete: if T::TO_USE.d_delete {
            Some(d_delete_callback::<T>)
        } else {
            None
        },
        d_init: None,
        d_release: None,
        d_prune: None,
        d_iput: None,
        d_dname: None,
        d_automount: None,
        d_manage: None,
        d_real: None,
    };

    pub(crate) const unsafe fn build() -> &'static bindings::dentry_operations {
        &Self::VTABLE
    }
}

/// A constant version where all values are to set to `false`, that is, all supported fields will
/// be set to null pointers.
pub const USE_NONE: ToUse = ToUse {
    d_revalidate: false,
    d_weak_revalidate: false,
    d_hash: false,
    d_compare: false,
    d_delete: false,
    d_init: false,
    d_release: false,
    d_prune: false,
    d_iput: false,
    d_dname: false,
    d_automount: false,
    d_manage: false,
    d_real: false,
};

pub struct ToUse {
    pub d_revalidate: bool,
    pub d_weak_revalidate: bool,
    pub d_hash: bool,
    pub d_compare: bool,
    pub d_delete: bool,
    pub d_init: bool,
    pub d_release: bool,
    pub d_prune: bool,
    pub d_iput: bool,
    pub d_dname: bool,
    pub d_automount: bool,
    pub d_manage: bool,
    pub d_real: bool,
}

#[macro_export]
macro_rules! declare_dentry_operations {
    () => {
        const TO_USE: $crate::fs::dentry::ToUse = $crate::fs::dentry::USE_NONE;
    };
    ($($i:ident),+) => {
        const TO_USE: kernel::fs::dentry::ToUse =
            $crate::fs::dentry::ToUse {
                $($i: true),+ ,
                ..$crate::fs::dentry::USE_NONE
            };
    };
}

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

pub trait DentryOperations {
    const TO_USE: ToUse;

    // fn d_revalidate(arg1: *mut dentry, arg2: c_types::c_uint) -> c_types::c_int {}
    // fn d_weak_revalidate(arg1: *mut dentry, arg2: c_types::c_uint) -> c_types::c_int {}
    // fn d_hash(arg1: *const dentry, arg2: *mut qstr) -> c_types::c_int {}
    // fn d_compare(
    //         arg1: *const dentry,
    //         arg2: c_types::c_uint,
    //         arg3: *const c_types::c_char,
    //         arg4: *const qstr,
    //     ) -> c_types::c_int {}
    // fn d_delete(arg1: *const dentry) -> c_types::c_int{}
    // fn d_init(arg1: *mut dentry) -> c_types::c_int{}
    // fn d_release(arg1: *mut dentry){}
    // fn d_prune(arg1: *mut dentry){}
    // fn d_iput(arg1: *mut dentry, arg2: *mut inode){}
    // fn d_dname(
    //         arg1: *mut dentry,
    //         arg2: *mut c_types::c_char,
    //         arg3: c_types::c_int,
    //     ) -> *mut c_types::c_char {}
    // fn d_automount(arg1: *mut path) -> *mut vfsmount{}
    // fn d_manage(arg1: *const path, arg2: bool_) -> c_types::c_int {}
    // fn d_real(arg1: *mut dentry, arg2: *const inode) -> *mut dentry {}

    // called when the last reference to a dentry is dropped and the dcache is
    // deciding whether or not to cache it. Return true to delete immediately, or
    // false to cache the dentry. Default is NULL which means to always cache a
    // reachable dentry. d_delete must be constant and idempotent.
    fn d_delete(_dentry: &Dentry) -> bool {
        false
    }
}
