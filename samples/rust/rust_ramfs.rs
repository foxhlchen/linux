// SPDX-License-Identifier: GPL-2.0

//! Rust printing macros sample

#![no_std]
#![feature(allocator_api, global_asm)]

use alloc::boxed::Box;
use alloc::vec::Vec;
use kernel::fs::*;
use kernel::prelude::*;
use kernel::str::CStr;
use kernel::{c_str, treedescr};
use kernel::{
    file::File,
    file_operations::{FileOpener, FileOperations},
    io_buffer::{IoBufferReader, IoBufferWriter},
    Error,
};

module! {
    type: Ramfs,
    name: b"rust_ramfs",
    author: b"Rust for Linux Contributors",
    description: b"Rust Ramfs",
    license: b"GPL v2",
}

static mut FS_HANDLE: Option<FSHandle> = None;

struct Ramfs;

#[derive(Default)]
struct FopsA;

#[derive(Default)]
struct FopsB;

impl FileOperations for FopsA {
    type Wrapper = Box<Self>;

    kernel::declare_file_operations!(read);

    fn read<T: IoBufferWriter>(&self, _: &File, data: &mut T, offset: u64) -> Result<usize> {
        // Succeed if the caller doesn't provide a buffer or if not at the start.
        if data.is_empty() || offset != 0 {
            return Ok(0);
        }
        pr_warn!("offset: {}", offset);

        // Write a one-byte 1 to the reader.
        data.write_slice(b"This is file A\n")?;
        Ok(b"This is file A\n".len())
    }
}

impl FileOperations for FopsB {
    type Wrapper = Box<Self>;

    kernel::declare_file_operations!(read);

    fn read<T: IoBufferWriter>(&self, _: &File, data: &mut T, offset: u64) -> Result<usize> {
        // Succeed if the caller doesn't provide a buffer or if not at the start.
        if data.is_empty() {
            return Err(Error::EINVAL);
        }
        pr_warn!("offset: {}", offset);

        // Write a one-byte 1 to the reader.
        data.write_slice(&['B' as u8; 1])?;
        Ok(1)
    }
}

impl FileSystem for Ramfs {
    const MOUNT_TYPE: MountType = MountType::Single;

    fn fill_super(sb: &mut SuperBlock, data: &CStr, silent: i32) -> Result<()> {
        let desc = treedescr! {
            "file_a", FopsA, S_IRUSR | S_IROTH;
            "file_b", FopsB, S_IRUSR;
        };

        simple_fill_super(sb, 17, &desc)?;

        Ok(())
    }
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
