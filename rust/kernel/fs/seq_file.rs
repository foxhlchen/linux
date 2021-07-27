// SPDX-License-Identifier: GPL-2.0

//! SeqFile.

use crate::bindings;
use crate::error::*;
use core::ptr;

pub struct SeqFile {
    c_seq_file: *mut bindings::seq_file,
}

impl SeqFile {
    pub fn default() -> SeqFile {
        SeqFile {
            c_seq_file: ptr::null_mut(),
        }
    }

    pub fn from_c_seq_file(c_seq_file: *mut bindings::seq_file) -> Result<Self> {
        if c_seq_file.is_null() {
            return Err(Error::EINVAL);
        }

        let mut seq_file = SeqFile::default();
        seq_file.c_seq_file = c_seq_file;

        Ok(seq_file)
    }

    pub fn to_c_seq_file(&self) -> *mut bindings::seq_file {
        self.c_seq_file
    }

    // TODO
    //pub fn printf()
}
