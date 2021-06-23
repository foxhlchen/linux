// SPDX-License-Identifier: GPL-2.0

//! Rust printing macros sample

#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;
use kernel::fs::*;
use kernel::c_str;

struct Ramfs {
}

module! {
    type: Ramfs,
    name: b"rust_ramfs",
    author: b"Rust for Linux Contributors",
    description: b"Rust Ramfs",
    license: b"GPL v2",
}

static mut FS_HANDLE: Option<FSHandle> = None;

impl FileSystem for Ramfs {
    const MOUNT_TYPE: MountType = MountType::Custom;
}

impl KernelModule for Ramfs {
    fn init() -> Result<Self> {
        unsafe { FS_HANDLE = Some(Ramfs::register_self(c_str!("rust_ramfs"), &THIS_MODULE)?) };
        pr_warn!("register rust fs");
        Ok(Self {})
    }
}

impl Drop for Ramfs {
    fn drop(&mut self) {
        unsafe {
            if let Some(fshd) = &mut FS_HANDLE {
                Ramfs::unregister_self(fshd);
            }
        }
    }
}
