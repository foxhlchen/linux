// SPDX-License-Identifier: GPL-2.0

//! SuperBlocks.

use crate::bindings;
use crate::error::*;
use core::ptr;
use super::c_types;
use crate::pr_warn;
use core::marker;

use super::kstatfs::KStatFs;
use super::dentry::Dentry;
use super::seq_file::SeqFile;
use super::inode::Inode;

// unsafe extern "C" fn alloc_inode<T: SuperBlockOperations>(sb: *mut bindings::super_block) -> *mut bindings::inode {}

unsafe extern "C" fn destroy_inode_callback<T: SuperBlockOperations>(c_inode: *mut bindings::inode) {
    let inode_rs = Inode::from_c_inode(c_inode);
    if inode_rs.is_err() {
        pr_warn!("Invalid inode in destroy_inode");
        return;
    }

    let mut inode = inode_rs.unwrap();
    T::destroy_inode(&mut inode);
}

unsafe extern "C" fn free_inode_callback<T: SuperBlockOperations>(c_inode: *mut bindings::inode) {
    let inode_rs = Inode::from_c_inode(c_inode);
    if inode_rs.is_err() {
        pr_warn!("Invalid inode in free_inode");
        return;
    }

    let mut inode = inode_rs.unwrap();
    T::free_inode(&mut inode);
}

// unsafe extern "C" fn dirty_inode<T: SuperBlockOperations>(arg1: *mut bindings::inode, flags: c_types::c_int) {}

// unsafe extern "C" fn write_inode<T: SuperBlockOperations>(
//     arg1: *mut bindings::inode,
//     wbc: *mut writeback_control,
// ) -> c_types::c_int {
// }


unsafe extern "C" fn drop_inode_callback<T: SuperBlockOperations>(c_inode: *mut bindings::inode) -> c_types::c_int {
    let inode_rs = Inode::from_c_inode(c_inode);
    if let Err(e) = inode_rs {
        return e.to_kernel_errno();
    }

    let mut inode = inode_rs.unwrap();
    T::drop_inode(&mut inode) as _
}

unsafe extern "C" fn evict_inode_callback<T: SuperBlockOperations>(c_inode: *mut bindings::inode) {
    let inode_rs = Inode::from_c_inode(c_inode);
    if inode_rs.is_err() {
        pr_warn!("Invalid inode in evict_inode");
        return;
    }

    let mut inode = inode_rs.unwrap();
    T::evict_inode(&mut inode);
}

// unsafe extern "C" fn put_super<T: SuperBlockOperations>(arg1: *mut bindings::super_block) {}

// unsafe extern "C" fn sync_fs<T: SuperBlockOperations>(
//     sb: *mut bindings::super_block,
//     wait: c_types::c_int,
// ) -> c_types::c_int {
// }

// unsafe extern "C" fn freeze_super<T: SuperBlockOperations>(arg1: *mut bindings::super_block) -> c_types::c_int {}

// unsafe extern "C" fn freeze_fs<T: SuperBlockOperations>(arg1: *mut bindings::super_block) -> c_types::c_int {}

// unsafe extern "C" fn thaw_super<T: SuperBlockOperations>(arg1: *mut bindings::super_block) -> c_types::c_int {}

// unsafe extern "C" fn unfreeze_fs<T: SuperBlockOperations>(arg1: *mut bindings::super_block) -> c_types::c_int {}

unsafe extern "C" fn statfs_callback<T: SuperBlockOperations>(
    c_dentry: *mut bindings::dentry,
    c_kstatfs: *mut bindings::kstatfs,
) -> c_types::c_int {
    let dentry_rs = Dentry::from_c_dentry(c_dentry);
    let kstatfs_rs = KStatFs::from_c_kstatfs(c_kstatfs);

    if let Err(e) = dentry_rs {
        return e.to_kernel_errno();
    }

    if let Err(e) = kstatfs_rs {
        return e.to_kernel_errno();
    }
    let mut dentry = dentry_rs.unwrap();
    let mut kstatfs = kstatfs_rs.unwrap();

    if let Err(e) = T::statfs(&mut dentry, &mut kstatfs) {
        return e.to_kernel_errno();
    }

    0
}

// unsafe extern "C" fn remount_fs<T: SuperBlockOperations>(
//     arg1: *mut bindings::super_block,
//     arg2: *mut c_types::c_int,
//     arg3: *mut c_types::c_char,
// ) -> c_types::c_int {
// }

// unsafe extern "C" fn umount_begin<T: SuperBlockOperations>(arg1: *mut bindings::super_block) {}

unsafe extern "C" fn show_options_callback<T: SuperBlockOperations>(
    c_seq_file: *mut bindings::seq_file,
    c_dentry: *mut bindings::dentry,
) -> c_types::c_int {
    let seq_file_rs = SeqFile::from_c_seq_file(c_seq_file);
    let dentry_rs = Dentry::from_c_dentry(c_dentry);

    if let Err(e) = seq_file_rs {
        return e.to_kernel_errno();
    }

    if let Err(e) = dentry_rs {
        return e.to_kernel_errno();
    }

    let mut seq_file = seq_file_rs.unwrap();
    let mut dentry = dentry_rs.unwrap();

    if let Err(e) = T::show_options(&mut seq_file, &mut dentry) {
        return e.to_kernel_errno();
    }

    0
}

// unsafe extern "C" fn show_devname<T: SuperBlockOperations>(
//     arg1: *mut seq_file,
//     arg2: *mut bindings::dentry,
// ) -> c_types::c_int {
// }

// unsafe extern "C" fn show_path<T: SuperBlockOperations>(
//     arg1: *mut seq_file,
//     arg2: *mut bindings::dentry,
// ) -> c_types::c_int {
// }

// unsafe extern "C" fn show_stats<T: SuperBlockOperations>(
//     arg1: *mut seq_file,
//     arg2: *mut bindings::dentry,
// ) -> c_types::c_int {
// }

// unsafe extern "C" fn quota_read<T: SuperBlockOperations>(
//     arg1: *mut bindings::super_block,
//     arg2: c_types::c_int,
//     arg3: *mut c_types::c_char,
//     arg4: usize,
//     arg5: loff_t,
// ) -> isize {
// }

// unsafe extern "C" fn quota_write<T: SuperBlockOperations>(
//     arg1: *mut bindings::super_block,
//     arg2: c_types::c_int,
//     arg3: *const c_types::c_char,
//     arg4: usize,
//     arg5: loff_t,
// ) -> isize {
// }

// unsafe extern "C" fn get_dquots<T: SuperBlockOperations>(arg1: *mut bindings::inode) -> *mut *mut dquot {}

// unsafe extern "C" fn bdev_try_to_free_page<T: SuperBlockOperations>(
//     arg1: *mut bindings::super_block,
//     arg2: *mut page,
//     arg3: gfp_t,
// ) -> c_types::c_int {
// }

// unsafe extern "C" fn nr_cached_objects<T: SuperBlockOperations>(
//     arg1: *mut bindings::super_block,
//     arg2: *mut shrink_control,
// ) -> c_types::c_long {
// }

// unsafe extern "C" fn free_cached_objects<T: SuperBlockOperations>(
//     arg1: *mut bindings::super_block,
//     arg2: *mut shrink_control,
// ) -> c_types::c_long {
// }


pub(crate) struct SuperBlockOperationsVtable<T> (marker::PhantomData<T>);

impl<T: SuperBlockOperations> SuperBlockOperationsVtable<T> {
    const VTABLE: bindings::super_operations = bindings::super_operations {
        alloc_inode: None,
        destroy_inode: if T::TO_USE.destroy_inode {
            Some(destroy_inode_callback::<T>)
        } else {
            None
        },
        free_inode: if T::TO_USE.free_inode {
            Some(free_inode_callback::<T>)
        } else {
            None
        },
        dirty_inode: None,
        write_inode: None,
        drop_inode: if T::TO_USE.drop_inode {
            Some(drop_inode_callback::<T>)
        } else {
            None
        },
        evict_inode: if T::TO_USE.evict_inode {
            Some(evict_inode_callback::<T>)
        } else {
            None
        },
        put_super: None,
        sync_fs: None,
        freeze_super: None,
        freeze_fs: None,
        thaw_super: None,
        unfreeze_fs: None,
        statfs: if T::TO_USE.statfs {
            Some(statfs_callback::<T>)
        } else {
            None
        },
        remount_fs: None,
        umount_begin: None,
        show_options: if T::TO_USE.show_options {
            Some(show_options_callback::<T>)
        } else {
            None
        },
        show_devname: None,
        show_path: None,
        show_stats: None,
        quota_read: None,
        quota_write: None,
        get_dquots: None,
        nr_cached_objects: None,
        free_cached_objects: None,
    };

    pub(crate) const unsafe fn build() -> &'static bindings::super_operations {
        &Self::VTABLE
    }
}

/// A constant version where all values are to set to `false`, that is, all supported fields will
/// be set to null pointers.
pub const USE_NONE: ToUse = ToUse {
    destroy_inode: false,
    free_inode: false,
    drop_inode: false,
    evict_inode: false,
    statfs: false,
    show_options: false,
};

pub struct ToUse {
    pub destroy_inode: bool,
    pub free_inode: bool,
    pub drop_inode: bool,
    pub evict_inode: bool,
    pub statfs: bool,
    pub show_options: bool,
}

#[macro_export]
macro_rules! declare_superblock_operations {
    () => {
        const TO_USE: $crate::fs::super_block::ToUse = $crate::fs::super_block::USE_NONE;
    };
    ($($i:ident),+) => {
        const TO_USE: kernel::fs::super_block::ToUse =
            $crate::fs::super_block::ToUse {
                $($i: true),+ ,
                ..$crate::fs::super_block::USE_NONE
            };
    };
}

pub struct SuperBlock {
    c_sb: *mut bindings::super_block,
}

impl SuperBlock {
    pub fn default() -> SuperBlock {
        SuperBlock {
            c_sb: ptr::null_mut(),
        }
    }

    pub fn from_c_super_block(c_sb: *mut bindings::super_block) -> Result<Self> {
        if c_sb.is_null() {
            return Err(Error::EINVAL);
        }

        let mut sb = SuperBlock::default();
        sb.c_sb = c_sb;

        Ok(sb)
    }

    pub fn to_c_super_block(&self) -> *mut bindings::super_block {
        self.c_sb
    }

    pub fn set_super_block_operations<T: SuperBlockOperations>(&self) {
        unsafe {
            (*self.c_sb).s_op = SuperBlockOperationsVtable::<T>::build();
        }
    }
}

pub trait SuperBlockOperations {
    const TO_USE: ToUse;
    // fn alloc_inode(sb: &SuperBlock) -> &Inode {}

    fn destroy_inode(_inode: &mut Inode) {}

    fn free_inode(_inode: &mut Inode) {}

    // fn dirty_inode(inode: &Inode, flags: i32) {}

    // fn write_inode(inode: &Inode, wbc: *mut writeback_control) -> Result {}

    fn drop_inode(_inode: &mut Inode) -> bool {
        true
    }

    fn evict_inode(_inode: &mut Inode) {}

    // fn put_super(sb: &SuperBlock) {}

    // fn sync_fs(sb: &SuperBlock, wait: i32) -> Result {}

    // fn freeze_super(arg1: &SuperBlock) -> Result {}

    // fn freeze_fs(arg1: &SuperBlock) -> Result {}

    // fn thaw_super(arg1: &SuperBlock) -> Result {}

    // fn unfreeze_fs(arg1: &SuperBlock) -> Result {}

    fn statfs(_dentry: &mut Dentry, _kfstatfs: &mut KStatFs) -> Result {
        Err(Error::EINVAL)
    }

    // fn remount_fs(
    //     arg1: &SuperBlock,
    //     arg2: *mut c_types::c_int,
    //     arg3: *mut c_types::c_char,
    // ) -> Result {
    // }

    // fn umount_begin(arg1: &SuperBlock) {}

    fn show_options(_seq_file: &mut SeqFile, _dentry: &mut Dentry) -> Result {
        Err(Error::EINVAL)
    }
}
