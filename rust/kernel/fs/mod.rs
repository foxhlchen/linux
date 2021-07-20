// SPDX-License-Identifier: GPL-2.0

//! File System Interfaces.

use super::file_operations::{FileOpenAdapter, FileOpener, FileOperationsVtable};
use crate::bindings;
use crate::bindings::{
    file, file_operations, file_system_type, mount_bdev, mount_nodev, mount_single,
    register_filesystem, unregister_filesystem,
};

use crate::str::*;
use crate::{c_str, c_types, error::Error, Result, ThisModule};
use alloc::boxed::Box;
use core::ptr;

pub use dentry::Dentry;
pub use inode::Inode;
pub use super_block::SuperBlock;
pub use kstatfs::KStatFs;

pub mod dentry;
pub mod inode;
pub mod super_block;
pub mod kstatfs;
pub mod seq_file;

unsafe extern "C" fn mount_callback<T: FileSystem>(
    fs_type: *mut file_system_type,
    flags: c_types::c_int,
    dev_name: *const c_types::c_char,
    data: *mut c_types::c_void,
) -> *mut bindings::dentry {
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
        return e.to_kernel_errno() as *mut bindings::dentry;
    }

    rt.unwrap().to_c_dentry()
}

unsafe extern "C" fn fill_super_callback<T: FileSystem>(
    sb: *mut bindings::super_block,
    data: *mut c_types::c_void,
    silent: c_types::c_int,
) -> c_types::c_int {
    let r_sb_rs = SuperBlock::from_c_super_block(sb);
    if let Err(e) = r_sb_rs {
        return e.to_kernel_errno();
    }

    let mut r_sb = r_sb_rs.unwrap();
    let r_data = if data.is_null() {
        c_str!("")
    } else {
        unsafe { CStr::from_char_ptr(data as *const c_types::c_char) }
    };

    let rs = T::fill_super(&mut r_sb, r_data, silent as i32);
    if let Err(e) = rs {
        return e.to_kernel_errno();
    }

    0
}

unsafe extern "C" fn kill_sb_callback<T: FileSystem>(sb: *mut bindings::super_block) {
    let r_sb_rs = SuperBlock::from_c_super_block(sb);

    if let Ok(r_sb) = r_sb_rs {
        T::kill_sb(&r_sb);
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

    pub fn to_c_fs_type(&self) -> *mut file_system_type {
        self.c_fs_type
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

impl<T: FileOpener<()>> FileOpenAdapter for T {
    type Arg = ();

    unsafe fn convert(_inode: *mut bindings::inode, _file: *mut file) -> *const Self::Arg {
        &()
    }
}

pub fn build_fops<A: FileOpenAdapter, T: FileOpener<A::Arg>>() -> &'static file_operations {
    return unsafe { FileOperationsVtable::<A, T>::build() };
}

// export tree_descr
pub use crate::bindings::tree_descr;

#[macro_export]
macro_rules! treedescr {
    (
        $($name:literal,$ops:ident,$mode:expr;)+
    ) => {
        {
            let mut v = Vec::<tree_descr>::new();

            // Because the root inode is 1, the files array must not contain an
	        // entry at index 1. We make them start at index 2.
            v.try_push(tree_descr::default())?; // index 0 skipped
            v.try_push(tree_descr::default())?; // index 1 skipped

            $(
                let mut tdesc = tree_descr::default();
                tdesc.name = c_str!($name).as_char_ptr();
                tdesc.ops = build_fops::<$ops, $ops>();
                tdesc.mode = $mode;

                v.try_push(tdesc)?;
            )+

            // Add ending mark
            let mut tdesc = tree_descr::default();
            tdesc.name = c_str!("").as_char_ptr();
            v.try_push(tdesc)?;

            v
        }
    };
    () => {
        {
            let mut v = Vec::<tree_descr>::new();

            let mut tdesc = tree_descr::default();
            tdesc.name = c_str!("").as_char_ptr();

            v.try_push(tdesc)?;
            v
        }
    };
}

pub fn simple_fill_super(sb: &mut SuperBlock, magic: usize, vec: &[tree_descr]) -> Result<()> {
    let rt = unsafe {
        crate::bindings::simple_fill_super(
            sb.to_c_super_block(),
            magic as c_types::c_ulong,
            vec.as_ptr(),
        )
    };
    if rt != 0 {
        return Err(Error::from_kernel_errno(rt));
    }

    Ok(())
}

pub fn simple_statfs(dentry: &mut Dentry, kstatfs: &mut KStatFs) -> Result {
    let rt = unsafe {
        bindings::simple_statfs(dentry.to_c_dentry(), kstatfs.to_c_kstatfs())
    };
    if rt != 0 {
        return Err(Error::from_kernel_errno(rt));
    }

    Ok(())
}


pub type FSHandle = Box<file_system_type>;

pub trait FileSystem: Sized + Sync {
    const MOUNT_TYPE: MountType;

    fn mount(_fs_type: &FSType, _flags: i32, _dev_name: &CStr, _data: &CStr) -> Result<Dentry> {
        Err(Error::EINVAL)
    }

    fn fill_super(_sb: &mut SuperBlock, _data: &CStr, _silent: i32) -> Result<()> {
        Err(Error::EINVAL)
    }

    fn kill_sb(_sb: &SuperBlock) {}

    fn register_self(name: &'static CStr, owner: &ThisModule) -> Result<FSHandle>
    where
        Self: Sized,
    {
        let mut c_fs_type = Box::try_new(file_system_type::default())?;
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
        let err = unsafe { unregister_filesystem(c_fs_type.as_mut() as *mut _) };
        if err != 0 {
            return Err(Error::from_kernel_errno(err));
        }

        Ok(())
    }
}
