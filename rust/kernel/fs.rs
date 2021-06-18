// SPDX-License-Identifier: GPL-2.0

//! File System Interfaces.

use crate::bindings::{
    dentry, file_system_type, inode, mount_bdev, mount_nodev, mount_single, super_block,
};
use crate::str::*;
use crate::{c_types, error::Error, Result};
use core::ptr;

unsafe extern "C" fn mount_callback<T: FileSystem>(
    fs_type: *mut file_system_type,
    flags: c_types::c_int,
    dev_name: *const c_types::c_char,
    data: *mut c_types::c_void,
) -> *mut dentry {
    let r_fs_type = FSType::from_c_fs_type(fs_type).unwrap();
    let r_dev_name = unsafe { CStr::from_char_ptr(dev_name) };
    let r_data = unsafe { CStr::from_char_ptr(data as *const c_types::c_char) };

    let rt: Result<Dentry> = match T::MOUNT_TYPE {
        MountType::Custom => T::mount(&r_fs_type, flags, r_dev_name, r_data),
        MountType::Single => Dentry::from_c_dentry(unsafe {
            mount_single(fs_type, flags, data, Some(fill_super_callback::<T>))
        }),
        MountType::BDev => Dentry::from_c_dentry(unsafe {
            mount_bdev(
                fs_type,
                flags,
                dev_name,
                data,
                Some(fill_super_callback::<T>),
            )
        }),
        MountType::NoDev => Dentry::from_c_dentry(unsafe {
            mount_nodev(fs_type, flags, data, Some(fill_super_callback::<T>))
        }),
    };

    if let Err(e) = rt {
        return e.to_kernel_errno() as *mut dentry;
    }

    rt.unwrap().to_c_dentry()
}

unsafe extern "C" fn fill_super_callback<T: FileSystem>(
    sb: *mut super_block,
    data: *mut c_types::c_void,
    silent: c_types::c_int,
) -> c_types::c_int {
    0
}

pub struct Dentry {
    c_dentry: *mut dentry,
}

impl Dentry {
    pub fn default() -> Dentry {
        Dentry {
            c_dentry: ptr::null_mut(),
        }
    }

    pub fn from_c_dentry(c_dentry: *mut dentry) -> Result<Self> {
        if c_dentry.is_null() {
            return Err(Error::EINVAL);
        }

        //TODO inc refcnt, and dec in dtor
        let mut d = Dentry::default();
        d.c_dentry = c_dentry;

        Ok(d)
    }

    pub fn to_c_dentry(&self) -> *mut dentry {
        return self.c_dentry;
    }
}

pub struct Inode {
    c_inode: *mut inode,
}

impl Inode {
    pub fn default() -> Inode {
        Inode {
            c_inode: ptr::null_mut(),
        }
    }

    pub fn from_c_inode(c_inode: *mut inode) -> Result<Self> {
        if c_inode.is_null() {
            return Err(Error::EINVAL);
        }

        //TODO inc refcnt, and dec in dtor
        let mut i = Inode::default();
        i.c_inode = c_inode;

        Ok(i)
    }

    pub fn to_c_inode(&self) -> *mut inode {
        return self.c_inode;
    }
}

pub struct SuperBlock {
    c_sb: *mut super_block,
}

impl SuperBlock {
    pub fn default() -> SuperBlock {
        SuperBlock {
            c_sb: ptr::null_mut(),
        }
    }

    pub fn from_c_super_block(c_sb: *mut super_block) -> Result<Self> {
        if c_sb.is_null() {
            return Err(Error::EINVAL);
        }

        let mut sb = SuperBlock::default();
        sb.c_sb = c_sb;

        Ok(sb)
    }
}

pub struct FSType {
    c_fs_type: *mut file_system_type,
}

impl FSType {
    pub fn default() -> FSType {
        FSType {
            c_fs_type: ptr::null_mut(),
        }
    }

    pub fn from_c_fs_type(c_fs_type: *mut file_system_type) -> Result<Self> {
        if c_fs_type.is_null() {
            return Err(Error::EINVAL);
        }

        let mut fs_type = FSType::default();
        fs_type.c_fs_type = c_fs_type;

        Ok(fs_type)
    }
}

pub enum MountType {
    // Call user provided mount function instead of [`fill_super()`]
    Custom,

    // Mount a filesystem residing on a block device
    BDev,

    // Mount a filesystem that is not backed by a device
    NoDev,

    // Mount a filesystem which shares the instance between all mounts
    Single,
}

pub trait FileSystem {
    const MOUNT_TYPE: MountType;

    fn mount(fs_type: &FSType, flags: i32, dev_name: &CStr, data: &CStr) -> Result<Dentry> {
        Err(Error::EINVAL)
    }

    fn fill_super(fs_type: &FSType, sb: &SuperBlock, data: &CStr) -> Result<()> {
        Err(Error::EINVAL)
    }
}

