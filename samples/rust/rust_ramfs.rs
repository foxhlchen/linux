// SPDX-License-Identifier: GPL-2.0

//! Rust printing macros sample

#![no_std]
#![feature(allocator_api, global_asm)]

use kernel::prelude::*;

struct Ramfs;

module! {
    type: Ramfs,
    name: b"rust_ramfs",
    author: b"Rust for Linux Contributors",
    description: b"Rust Ramfs",
    license: b"GPL v2",
}

impl KernelModule for Ramfs {
    fn init() -> Result<Self> {
        Ok(Self {})
    }
}

impl Drop for Ramfs {
    fn drop(&mut self) {}
}
