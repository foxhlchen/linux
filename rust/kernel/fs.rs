// SPDX-License-Identifier: GPL-2.0

//! File System Interfaces.

use crate::bindings::{
    dentry, file_system_type, inode, mount_bdev, mount_nodev, mount_single, super_block, register_filesystem,
    unregister_filesystem
};
use crate::str::*;
use crate::{c_types, error::Error, Result, ThisModule, c_str};
use core::ptr;
use alloc::boxed::Box;

unsafe extern "C" fn mount_callback<T: FileSystem>(
    fs_type: *mut file_system_type,
    flags: c_types::c_int,
    dev_name: *const c_types::c_char,
    data: *mut c_types::c_void,
) -> *mut dentry {
    let r_fs_type = FSType::from_c_fs_type(fs_type).unwrap();
    let r_dev_name = if dev_name.is_null() {
        c_str!("")
    } else {
        unsafe { CStr::from_char_ptr(dev_name) }
    };
    let r_data = if data.is_null() { 
        c_str!("")
    } else { 
        unsafe { CStr::from_char_ptr(data as *const c_types::c_char) }
    };

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
        //TODO wrap ETR_PTR
        return e.to_kernel_errno() as *mut dentry;
    }

    rt.unwrap().to_c_dentry()
}

unsafe extern "C" fn fill_super_callback<T: FileSystem>(
    sb: *mut super_block,
    data: *mut c_types::c_void,
    silent: c_types::c_int,
) -> c_types::c_int {
    let r_sb_rs = SuperBlock::from_c_super_block(sb);
    if let Err(e) = r_sb_rs {
        return e.to_kernel_errno();
    }

    let r_sb = r_sb_rs.unwrap();
    let r_data = if data.is_null() { 
        c_str!("")
    } else { 
        unsafe { CStr::from_char_ptr(data as *const c_types::c_char) }
    };

    let rs = T::fill_super(&r_sb, r_data, silent as i32);
    if let Err(e) = rs {
        return e.to_kernel_errno();
    }

    0
}

unsafe extern "C" fn kill_sb_callback<T: FileSystem>(sb: *mut super_block) {
    let r_sb_rs = SuperBlock::from_c_super_block(sb);

    if let Ok(r_sb) = r_sb_rs {
        T::kill_sb(&r_sb);
    }
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

pub type FSHandle = Box::<file_system_type>;

pub trait FileSystem: Sized + Sync {
    const MOUNT_TYPE: MountType;

    fn mount(fs_type: &FSType, flags: i32, dev_name: &CStr, data: &CStr) -> Result<Dentry> {
        crate::pr_warn!("mount fs");
        Err(Error::EINVAL)
    }

    fn fill_super(sb: &SuperBlock, data: &CStr, silent: i32) -> Result<()> {
        crate::pr_warn!("fill super");
        Err(Error::EINVAL)
    }

    fn kill_sb(sb: &SuperBlock) {

    }

    fn register_self(name: &'static CStr, owner: &ThisModule) -> Result<FSHandle> where Self: Sized {
        let mut c_fs_type = Box::new(file_system_type::default());
        c_fs_type.mount = Some(mount_callback::<Self>);
        c_fs_type.kill_sb = Some(kill_sb_callback::<Self>);
        c_fs_type.owner = owner.0;
        c_fs_type.name = name.as_char_ptr();

        let err = unsafe { register_filesystem(c_fs_type.as_mut() as *mut _) };
        if err != 0 {
            return Err(Error::from_kernel_errno(err));
        }

        Ok(c_fs_type)
    }

    fn unregister_self(c_fs_type: &mut FSHandle) -> Result<()> {
        let err = unsafe{ unregister_filesystem(c_fs_type.as_mut() as *mut _) };
        if err != 0 {
            return Err(Error::from_kernel_errno(err));
        }

        Ok(())
    }
}
