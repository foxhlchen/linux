// SPDX-License-Identifier: GPL-2.0

//! SuperBlocks.

use crate::bindings;
use crate::error::*;
use core::ptr;
use super::c_types;

use super::kstatfs::KStatFs;
use super::dentry::Dentry;
use super::seq_file::SeqFile;
use super::inode::Inode;

// unsafe extern "C" fn alloc_inode<T: SuperBlockOperations>(sb: *mut bindings::super_block) -> *mut bindings::inode {}

unsafe extern "C" fn destroy_inode<T: SuperBlockOperations>(arg1: *mut bindings::inode) {}

unsafe extern "C" fn free_inode<T: SuperBlockOperations>(arg1: *mut bindings::inode) {}

// unsafe extern "C" fn dirty_inode<T: SuperBlockOperations>(arg1: *mut bindings::inode, flags: c_types::c_int) {}

// unsafe extern "C" fn write_inode<T: SuperBlockOperations>(
//     arg1: *mut bindings::inode,
//     wbc: *mut writeback_control,
// ) -> c_types::c_int {
// }


unsafe extern "C" fn drop_inode<T: SuperBlockOperations>(arg1: *mut bindings::inode) -> c_types::c_int {
    0
}

unsafe extern "C" fn evict_inode<T: SuperBlockOperations>(arg1: *mut bindings::inode) {}

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

unsafe extern "C" fn statfs<T: SuperBlockOperations>(
    arg1: *mut bindings::dentry,
    arg2: *mut bindings::kstatfs,
) -> c_types::c_int {
    0
}

// unsafe extern "C" fn remount_fs<T: SuperBlockOperations>(
//     arg1: *mut bindings::super_block,
//     arg2: *mut c_types::c_int,
//     arg3: *mut c_types::c_char,
// ) -> c_types::c_int {
// }

// unsafe extern "C" fn umount_begin<T: SuperBlockOperations>(arg1: *mut bindings::super_block) {}

unsafe extern "C" fn show_options<T: SuperBlockOperations>(
    arg1: *mut bindings::seq_file,
    arg2: *mut bindings::dentry,
) -> c_types::c_int {
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


#[derive(Default)]
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
        const TO_USE: $crate::fs::super_block::ToUse = Default::default();
    };
    ($($i:ident),+) => {
        const TO_USE: kernel::fs::super_block::ToUse =
            $crate::fs::super_block::ToUse {
                $($i: true),+ ,
                ..Default::default()
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
}

pub trait SuperBlockOperations {
    const TO_USE: ToUse;
    // fn alloc_inode(sb: &SuperBlock) -> &Inode {}

    fn destroy_inode(inode: &Inode) {}

    fn free_inode(inode: &Inode) {}

    // fn dirty_inode(inode: &Inode, flags: i32) {}

    // fn write_inode(inode: &Inode, wbc: *mut writeback_control) -> Result {}

    fn drop_inode(inode: &Inode) -> bool {
        true
    }

    fn evict_inode(inode: &Inode) {}

    // fn put_super(sb: &SuperBlock) {}

    // fn sync_fs(sb: &SuperBlock, wait: i32) -> Result {}

    // fn freeze_super(arg1: &SuperBlock) -> Result {}

    // fn freeze_fs(arg1: &SuperBlock) -> Result {}

    // fn thaw_super(arg1: &SuperBlock) -> Result {}

    // fn unfreeze_fs(arg1: &SuperBlock) -> Result {}

    fn statfs(_dentry: &Dentry, _kfstatfs: KStatFs) -> Result {
        Err(Error::EINVAL)
    }

    // fn remount_fs(
    //     arg1: &SuperBlock,
    //     arg2: *mut c_types::c_int,
    //     arg3: *mut c_types::c_char,
    // ) -> Result {
    // }

    // fn umount_begin(arg1: &SuperBlock) {}

    fn show_options(seq_file: &SeqFile, dentry: &Dentry) -> Result {
        Err(Error::EINVAL)
    }
}
