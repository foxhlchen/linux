// SPDX-License-Identifier: GPL-2.0

//! Inode.

use super::c_types;
use super::dentry::Dentry;
use super::types::DevType;
use super::types::IAttr;
use super::types::KStat;
use super::types::Path;
use super::types::UMode;
use super::user_ns::UserNameSpace;
use crate::bindings;
use crate::error::*;
use crate::pr_warn;
use crate::str::*;
use core::marker;
use core::ptr;

unsafe extern "C" fn lookup<T: InodeOperations>(
    c_inode: *mut bindings::inode,
    c_dentry: *mut bindings::dentry,
    flags: c_types::c_uint,
) -> *mut bindings::dentry {
    let inode_rs = Inode::from_c_inode(c_inode);
    if let Err(e) = inode_rs {
        pr_warn!("Invalid inode in destroy_inode");
        return e.to_kernel_errno() as _;
    }

    let dentry_rs = Dentry::from_c_dentry(c_dentry);
    if let Err(e) = dentry_rs {
        pr_warn!("Invalid inode in destroy_inode");
        return e.to_kernel_errno() as _;
    }

    let mut inode = inode_rs.unwrap();
    let mut dentry = dentry_rs.unwrap();

    let rs = T::lookup(&mut inode, &mut dentry, flags as u32);
    if let Err(e) = rs {
        return e.to_kernel_errno() as _;
    }

    rs.unwrap().to_c_dentry()
}

// unsafe extern "C" fn get_link<T: InodeOperations>(
//     arg1: *mut dentry,
//     arg2: *mut inode,
//     arg3: *mut delayed_call,
// ) -> *const c_types::c_char {}
// unsafe extern "C" fn permission<T: InodeOperations>(
//     arg1: *mut user_namespace,
//     arg2: *mut inode,
//     arg3: c_types::c_int,
// ) -> c_types::c_int {}
// unsafe extern "C" fn get_acl<T: InodeOperations>(arg1: *mut inode, arg2: c_types::c_int) -> *mut posix_acl {}
// unsafe extern "C" fn readlink<T: InodeOperations>(
//     arg1: *mut dentry,
//     arg2: *mut c_types::c_char,
//     arg3: c_types::c_int,
// ) -> c_types::c_int {}

unsafe extern "C" fn create<T: InodeOperations>(
    c_user_ns: *mut bindings::user_namespace,
    c_inode: *mut bindings::inode,
    c_dentry: *mut bindings::dentry,
    c_mode: bindings::umode_t,
    c_excl: bindings::bool_,
) -> c_types::c_int {
    let user_ns_rs = UserNameSpace::from_c_user_namespace(c_user_ns);
    if let Err(e) = user_ns_rs {
        return e.to_kernel_errno();
    }

    let inode_rs = Inode::from_c_inode(c_inode);
    if let Err(e) = inode_rs {
        return e.to_kernel_errno();
    }

    let dentry_rs = Dentry::from_c_dentry(c_dentry);
    if let Err(e) = dentry_rs {
        return e.to_kernel_errno();
    }

    let mut user_ns = user_ns_rs.unwrap();
    let mut inode = inode_rs.unwrap();
    let mut dentry = dentry_rs.unwrap();
    let mode = c_mode as UMode;
    let excl = c_excl as bool;

    let rs = T::create(&mut user_ns, &mut inode, &mut dentry, mode, excl);
    if let Err(e) = rs {
        return e.to_kernel_errno();
    }

    0
}

unsafe extern "C" fn link<T: InodeOperations>(
    c_old_dentry: *mut bindings::dentry,
    c_dir: *mut bindings::inode,
    c_dentry: *mut bindings::dentry,
) -> c_types::c_int {
    let old_dentry_rs = Dentry::from_c_dentry(c_old_dentry);
    if let Err(e) = old_dentry_rs {
        return e.to_kernel_errno();
    }

    let dir_rs = Inode::from_c_inode(c_dir);
    if let Err(e) = dir_rs {
        return e.to_kernel_errno();
    }

    let dentry_rs = Dentry::from_c_dentry(c_dentry);
    if let Err(e) = dentry_rs {
        return e.to_kernel_errno();
    }

    let mut old_dentry = old_dentry_rs.unwrap();
    let mut dir = dir_rs.unwrap();
    let mut dentry = dentry_rs.unwrap();

    let rs = T::link(&mut old_dentry, &mut dir, &mut dentry);
    if let Err(e) = rs {
        return e.to_kernel_errno();
    }

    0
}

unsafe extern "C" fn unlink<T: InodeOperations>(
    c_inode: *mut bindings::inode,
    c_dentry: *mut bindings::dentry,
) -> c_types::c_int {
    let inode_rs = Inode::from_c_inode(c_inode);
    if let Err(e) = inode_rs {
        return e.to_kernel_errno();
    }

    let dentry_rs = Dentry::from_c_dentry(c_dentry);
    if let Err(e) = dentry_rs {
        return e.to_kernel_errno();
    }

    let mut inode = inode_rs.unwrap();
    let mut dentry = dentry_rs.unwrap();

    let rs = T::unlink(&mut inode, &mut dentry);
    if let Err(e) = rs {
        return e.to_kernel_errno();
    }

    0
}

unsafe extern "C" fn symlink<T: InodeOperations>(
    c_user_ns: *mut bindings::user_namespace,
    c_inode: *mut bindings::inode,
    c_dentry: *mut bindings::dentry,
    c_symname: *const c_types::c_char,
) -> c_types::c_int {
    let user_ns_rs = UserNameSpace::from_c_user_namespace(c_user_ns);
    if let Err(e) = user_ns_rs {
        return e.to_kernel_errno();
    }

    let inode_rs = Inode::from_c_inode(c_inode);
    if let Err(e) = inode_rs {
        return e.to_kernel_errno();
    }

    let dentry_rs = Dentry::from_c_dentry(c_dentry);
    if let Err(e) = dentry_rs {
        return e.to_kernel_errno();
    }

    let mut user_ns = user_ns_rs.unwrap();
    let mut inode = inode_rs.unwrap();
    let mut dentry = dentry_rs.unwrap();
    let sym_name = unsafe { CStr::from_char_ptr(c_symname) };

    let rs = T::symlink(&mut user_ns, &mut inode, &mut dentry, sym_name);
    if let Err(e) = rs {
        return e.to_kernel_errno();
    }

    0
}

unsafe extern "C" fn mkdir<T: InodeOperations>(
    c_user_ns: *mut bindings::user_namespace,
    c_inode: *mut bindings::inode,
    c_dentry: *mut bindings::dentry,
    c_mode: bindings::umode_t,
) -> c_types::c_int {
    let user_ns_rs = UserNameSpace::from_c_user_namespace(c_user_ns);
    if let Err(e) = user_ns_rs {
        return e.to_kernel_errno();
    }

    let inode_rs = Inode::from_c_inode(c_inode);
    if let Err(e) = inode_rs {
        return e.to_kernel_errno();
    }

    let dentry_rs = Dentry::from_c_dentry(c_dentry);
    if let Err(e) = dentry_rs {
        return e.to_kernel_errno();
    }

    let mut user_ns = user_ns_rs.unwrap();
    let mut inode = inode_rs.unwrap();
    let mut dentry = dentry_rs.unwrap();
    let mode = c_mode as UMode;

    let rs = T::mkdir(&mut user_ns, &mut inode, &mut dentry, mode);
    if let Err(e) = rs {
        return e.to_kernel_errno();
    }

    0
}

unsafe extern "C" fn rmdir<T: InodeOperations>(
    c_inode: *mut bindings::inode,
    c_dentry: *mut bindings::dentry,
) -> c_types::c_int {
    let inode_rs = Inode::from_c_inode(c_inode);
    if let Err(e) = inode_rs {
        return e.to_kernel_errno();
    }

    let dentry_rs = Dentry::from_c_dentry(c_dentry);
    if let Err(e) = dentry_rs {
        return e.to_kernel_errno();
    }

    let mut inode = inode_rs.unwrap();
    let mut dentry = dentry_rs.unwrap();

    let rs = T::rmdir(&mut inode, &mut dentry);
    if let Err(e) = rs {
        return e.to_kernel_errno();
    }

    0
}

unsafe extern "C" fn mknod<T: InodeOperations>(
    c_user_ns: *mut bindings::user_namespace,
    c_inode: *mut bindings::inode,
    c_dentry: *mut bindings::dentry,
    c_mode: bindings::umode_t,
    c_rdev: bindings::dev_t,
) -> c_types::c_int {
    let user_ns_rs = UserNameSpace::from_c_user_namespace(c_user_ns);
    if let Err(e) = user_ns_rs {
        return e.to_kernel_errno();
    }

    let inode_rs = Inode::from_c_inode(c_inode);
    if let Err(e) = inode_rs {
        return e.to_kernel_errno();
    }

    let dentry_rs = Dentry::from_c_dentry(c_dentry);
    if let Err(e) = dentry_rs {
        return e.to_kernel_errno();
    }

    let mut user_ns = user_ns_rs.unwrap();
    let mut inode = inode_rs.unwrap();
    let mut dentry = dentry_rs.unwrap();
    let mode = c_mode as UMode;
    let rdev = c_rdev as DevType;

    let rs = T::mknod(&mut user_ns, &mut inode, &mut dentry, mode, rdev);
    if let Err(e) = rs {
        return e.to_kernel_errno();
    }

    0
}

unsafe extern "C" fn rename<T: InodeOperations>(
    c_user_ns: *mut bindings::user_namespace,
    c_old_dir: *mut bindings::inode,
    c_old_dentry: *mut bindings::dentry,
    c_new_dir: *mut bindings::inode,
    c_new_dentry: *mut bindings::dentry,
    c_flags: c_types::c_uint,
) -> c_types::c_int {
    let user_ns_rs = UserNameSpace::from_c_user_namespace(c_user_ns);
    if let Err(e) = user_ns_rs {
        return e.to_kernel_errno();
    }

    let old_dir_rs = Inode::from_c_inode(c_old_dir);
    if let Err(e) = old_dir_rs {
        return e.to_kernel_errno();
    }

    let old_dentry_rs = Dentry::from_c_dentry(c_old_dentry);
    if let Err(e) = old_dentry_rs {
        return e.to_kernel_errno();
    }

    let new_dir_rs = Inode::from_c_inode(c_new_dir);
    if let Err(e) = new_dir_rs {
        return e.to_kernel_errno();
    }

    let new_dentry_rs = Dentry::from_c_dentry(c_new_dentry);
    if let Err(e) = new_dentry_rs {
        return e.to_kernel_errno();
    }

    let mut user_ns = user_ns_rs.unwrap();
    let mut old_dir = old_dir_rs.unwrap();
    let mut old_dentry = old_dentry_rs.unwrap();
    let mut new_dir = new_dir_rs.unwrap();
    let mut new_dentry = new_dentry_rs.unwrap();
    let flags = c_flags as u32;

    let rs = T::rename(
        &mut user_ns,
        &mut old_dir,
        &mut old_dentry,
        &mut new_dir,
        &mut new_dentry,
        flags,
    );
    if let Err(e) = rs {
        return e.to_kernel_errno();
    }

    0
}

unsafe extern "C" fn setattr<T: InodeOperations>(
    c_user_ns: *mut bindings::user_namespace,
    c_dentry: *mut bindings::dentry,
    c_attr: *mut bindings::iattr,
) -> c_types::c_int {
    let user_ns_rs = UserNameSpace::from_c_user_namespace(c_user_ns);
    if let Err(e) = user_ns_rs {
        return e.to_kernel_errno();
    }

    let dentry_rs = Dentry::from_c_dentry(c_dentry);
    if let Err(e) = dentry_rs {
        return e.to_kernel_errno();
    }

    if c_attr.is_null() {
        return Error::EINVAL.to_kernel_errno();
    }

    let mut user_ns = user_ns_rs.unwrap();
    let mut dentry = dentry_rs.unwrap();
    let mut attr = unsafe { *c_attr };

    let rs = T::setattr(&mut user_ns, &mut dentry, &mut attr);
    if let Err(e) = rs {
        return e.to_kernel_errno();
    }

    0
}

unsafe extern "C" fn getattr<T: InodeOperations>(
    c_user_ns: *mut bindings::user_namespace,
    c_path: *const bindings::path,
    c_kstat: *mut bindings::kstat,
    c_request_mask: bindings::u32_,
    c_query_flags: c_types::c_uint,
) -> c_types::c_int {
    let user_ns_rs = UserNameSpace::from_c_user_namespace(c_user_ns);
    if let Err(e) = user_ns_rs {
        return e.to_kernel_errno();
    }

    if c_path.is_null() {
        return Error::EINVAL.to_kernel_errno();
    }

    if c_kstat.is_null() {
        return Error::EINVAL.to_kernel_errno();
    }

    let mut user_ns = user_ns_rs.unwrap();
    let path = unsafe { *c_path };
    let mut kstat = unsafe { *c_kstat };
    let request_mask = c_request_mask as u32;
    let query_flags = c_query_flags as u32;

    let rs = T::getattr(&mut user_ns, &path, &mut kstat, request_mask, query_flags);
    if let Err(e) = rs {
        return e.to_kernel_errno();
    }

    0
}

// unsafe extern "C" fn listxattr<T: InodeOperations>(arg1: *mut dentry, arg2: *mut c_types::c_char, arg3: usize) -> isize {}
// unsafe extern "C" fn fiemap<T: InodeOperations>(
//     arg1: *mut inode,
//     arg2: *mut fiemap_extent_info,
//     start: u64_,
//     len: u64_,
// ) -> c_types::c_int {}
// unsafe extern "C" fn update_time<T: InodeOperations>(
//     arg1: *mut inode,
//     arg2: *mut timespec64,
//     arg3: c_types::c_int,
// ) -> c_types::c_int {}
// unsafe extern "C" fn atomic_open<T: InodeOperations>(
//     arg1: *mut inode,
//     arg2: *mut dentry,
//     arg3: *mut file,
//     open_flag: c_types::c_uint,
//     create_mode: umode_t,
// ) -> c_types::c_int {}

// unsafe extern "C" fn tmpfile<T: InodeOperations>(
//     arg1: *mut bindings::user_namespace,
//     arg2: *mut bindings::inode,
//     arg3: *mut bindings::dentry,
//     arg4: bindings::umode_t,
// ) -> c_types::c_int {

// }

// unsafe extern "C" fn set_acl<T: InodeOperations>(
//     arg1: *mut user_namespace,
//     arg2: *mut inode,
//     arg3: *mut posix_acl,
//     arg4: c_types::c_int,
// ) -> c_types::c_int {}
// unsafe extern "C" fn fileattr_set<T: InodeOperations>(
//     mnt_userns: *mut user_namespace,
//     dentry: *mut dentry,
//     fa: *mut fileattr,
// ) -> c_types::c_int {}
// unsafe extern "C" fn fileattr_get<T: InodeOperations>(dentry: *mut dentry, fa: *mut fileattr) -> c_types::c_int {}

pub(crate) struct InodeOperationsVtable<T>(marker::PhantomData<T>);

impl<T: InodeOperations> InodeOperationsVtable<T> {
    const VTABLE: bindings::inode_operations = bindings::inode_operations {
        lookup: None,
        get_link: None,
        permission: None,
        get_acl: None,
        readlink: None,
        create: None,
        link: None,
        unlink: None,
        symlink: None,
        mkdir: None,
        rmdir: None,
        mknod: None,
        rename: None,
        setattr: None,
        getattr: None,
        listxattr: None,
        fiemap: None,
        update_time: None,
        atomic_open: None,
        tmpfile: None,
        set_acl: None,
        fileattr_set: None,
        fileattr_get: None,
    };

    pub(crate) const unsafe fn build() -> &'static bindings::inode_operations {
        &Self::VTABLE
    }
}

/// A constant version where all values are to set to `false`, that is, all supported fields will
/// be set to null pointers.
pub const USE_NONE: ToUse = ToUse {
    lookup: false,
    get_link: false,
    permission: false,
    get_acl: false,
    readlink: false,
    create: false,
    link: false,
    unlink: false,
    symlink: false,
    mkdir: false,
    rmdir: false,
    mknod: false,
    rename: false,
    setattr: false,
    getattr: false,
    listxattr: false,
    fiemap: false,
    update_time: false,
    atomic_open: false,
    tmpfile: false,
    set_acl: false,
    fileattr_set: false,
    fileattr_get: false,
};

pub struct ToUse {
    pub lookup: bool,
    pub get_link: bool,
    pub permission: bool,
    pub get_acl: bool,
    pub readlink: bool,
    pub create: bool,
    pub link: bool,
    pub unlink: bool,
    pub symlink: bool,
    pub mkdir: bool,
    pub rmdir: bool,
    pub mknod: bool,
    pub rename: bool,
    pub setattr: bool,
    pub getattr: bool,
    pub listxattr: bool,
    pub fiemap: bool,
    pub update_time: bool,
    pub atomic_open: bool,
    pub tmpfile: bool,
    pub set_acl: bool,
    pub fileattr_set: bool,
    pub fileattr_get: bool,
}

#[macro_export]
macro_rules! declare_inode_operations {
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

pub struct Inode {
    c_inode: *mut bindings::inode,
}

impl Inode {
    pub fn default() -> Inode {
        Inode {
            c_inode: ptr::null_mut(),
        }
    }

    pub fn from_c_inode(c_inode: *mut bindings::inode) -> Result<Self> {
        if c_inode.is_null() {
            return Err(Error::EINVAL);
        }

        //TODO inc refcnt, and dec in dtor
        let mut i = Inode::default();
        i.c_inode = c_inode;

        Ok(i)
    }

    pub fn to_c_inode(&self) -> *mut bindings::inode {
        self.c_inode
    }
}

pub trait InodeOperations {
    fn lookup(_inode: &mut Inode, _dentry: &mut Dentry, _flags: u32) -> Result<Dentry> {
        Err(Error::EINVAL)
    }

    // fn get_link(
    //         dentry: &mut Dentry,
    //         inode: &mut Inode,
    //         arg3: *mut delayed_call,
    //     ) -> *const c_types::c_char {}
    // fn permission(
    //         arg1: *mut user_namespace,
    //         inode: &mut Inode,
    //         arg3: c_types::c_int,
    //     ) -> c_types::c_int {}
    // fn get_acl(inode: &mut Inode, arg2: c_types::c_int) -> *mut posix_acl {}
    // fn readlink(
    //         dentry: &mut Dentry,
    //         arg2: *mut c_types::c_char,
    //         arg3: c_types::c_int,
    //     ) -> c_types::c_int {}

    // fill inode to dentry
    fn create(
        _mnt_userns: &mut UserNameSpace,
        _inode: &mut Inode,
        _dentry: &mut Dentry,
        _mode: UMode,
        _excl: bool,
    ) -> Result {
        Err(Error::EINVAL)
    }

    fn link(_old: &mut Dentry, _dir: &mut Inode, _new: &mut Dentry) -> Result {
        Err(Error::EINVAL)
    }

    fn unlink(_dir: &mut Inode, _dentry: &mut Dentry) -> Result {
        Err(Error::EINVAL)
    }

    fn symlink(
        _mnt_userns: &mut UserNameSpace,
        _dir: &mut Inode,
        _dentry: &mut Dentry,
        _sym_name: &CStr,
    ) -> Result {
        Err(Error::EINVAL)
    }

    fn mkdir(
        _mnt_userns: &mut UserNameSpace,
        _inode: &mut Inode,
        _dentry: &mut Dentry,
        _mode: UMode,
    ) -> Result {
        Err(Error::EINVAL)
    }

    fn rmdir(_inode: &mut Inode, _dentry: &mut Dentry) -> Result {
        Err(Error::EINVAL)
    }

    fn mknod(
        _mnt_userns: &mut UserNameSpace,
        _inode: &mut Inode,
        _dentry: &mut Dentry,
        _mode: UMode,
        _rdev: DevType,
    ) -> Result {
        Err(Error::EINVAL)
    }

    fn rename(
        _mnt_userns: &mut UserNameSpace,
        _old_dir: &mut Inode,
        _old_dentry: &mut Dentry,
        _new_dir: &mut Inode,
        _new_dentry: &mut Dentry,
        _flags: u32,
    ) -> Result {
        Err(Error::EINVAL)
    }

    fn setattr(
        _mnt_userns: &mut UserNameSpace,
        _dentry: &mut Dentry,
        _iattr: &mut IAttr,
    ) -> Result {
        Err(Error::EINVAL)
    }

    fn getattr(
        _mnt_userns: &mut UserNameSpace,
        _path: &Path,
        _kstat: &mut KStat,
        _mask: u32,
        _query_flags: u32,
    ) -> Result {
        Err(Error::EINVAL)
    }

    // fn listxattr(dentry: &mut Dentry, arg2: *mut c_types::c_char, arg3: usize) -> isize {}
    // fn fiemap(
    //         inode: &mut Inode,
    //         arg2: *mut fiemap_extent_info,
    //         start: u64_,
    //         len: u64_,
    //     ) -> c_types::c_int {}
    // fn update_time(
    //         inode: &mut Inode,
    //         arg2: *mut timespec64,
    //         arg3: c_types::c_int,
    //     ) -> c_types::c_int {}
    // fn atomic_open(
    //         inode: &mut Inode,
    //         dentry: &mut Dentry,
    //         arg3: *mut file,
    //         open_flag: c_types::c_uint,
    //         create_mode: umode_t,
    //     ) -> c_types::c_int {}
    // fn tmpfile(
    //         arg1: &mut UserNameSpace,
    //         inode: &mut Inode,
    //         dentry: &mut Dentry,
    //         arg4: umode_t,
    //     ) -> c_types::c_int {}
    // fn set_acl(
    //         arg1: &mut UserNameSpace,
    //         inode: &mut Inode,
    //         arg3: *mut posix_acl,
    //         arg4: c_types::c_int,
    //     ) -> c_types::c_int {}
    // fn fileattr_set(
    //         mnt_userns: &mut UserNameSpace,
    //         dentry: *mut dentry,
    //         fa: *mut fileattr,
    //     ) -> c_types::c_int {}
    // fn fileattr_get(dentry: *mut dentry, fa: *mut fileattr) -> c_types::c_int {}
}
