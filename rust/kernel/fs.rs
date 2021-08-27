pub mod address_space;
pub mod address_space_operations;
pub mod dentry;
pub mod inode;
pub mod inode_operations;
pub mod kiocb;
pub mod libfs_functions;
pub mod super_block;
pub mod super_operations;

use core::ptr;

use crate::{
    bindings, c_types::*, error::from_kernel_err_ptr, fs::super_block::SuperBlock, print::ExpectK,
    ret_err_ptr, str::CStr, types::FileSystemFlags, Result,
};

pub trait BuildVtable<T> {
    fn build_vtable() -> &'static T;
}
#[macro_export]
macro_rules! declare_c_vtable {
    ($O:ident, $T:ty, $val:expr $(,)?) => {
        pub struct $O;
        impl $crate::fs::BuildVtable<$T> for $O {
            fn build_vtable() -> &'static $T {
                unsafe { &($val) }
            }
        }
    };
}

pub struct Registration<T: FileSystemBase> {
    phantom: PhantomData<T>,
    fs_type: FileSystemType,
}

//Pin self
impl Registration<T: FileSystemBase> {

    fn new(fs_type: FileSystemType ) -> Self {
        Self {
            PhantomData,
            fs_type
        }
    }

    fn new_pinned() -> Result<Pin<Box<Self>>> {
        let mut c_fs_type = file_system_type::default();
        c_fs_type.mount = Some(mount_callback::<Self>);
        c_fs_type.kill_sb = Some(kill_superblock_callback::<Self>);
        c_fs_type.owner = T::OWNER;
        c_fs_type.name = T::NAME.as_char_ptr();
        
        Ok(Pin::from(Box::try_new(Self::new(c_fs_type))?))
    }

    fn register(&self) -> Result {
        let err = unsafe { register_filesystem(self.get_mut().fs_type as *mut _) };
        if err != 0 {
            return Err(Error::from_kernel_errno(err));
        }

        Ok(())
    }

    fn unregister() -> Result {
        let err = unsafe { unregister_filesystem(self.get_mut() as *mut _) };
        if err != 0 {
            return Err(Error::from_kernel_errno(err));
        }

        Ok(())
    }
}

pub type FileSystemType = bindings::file_system_type;
pub type PinnedFSType = Pin<Box<FileSystemType>>

pub trait FileSystemBase {
    type MountOptions = c_void;

    const NAME: &'static CStr;
    const FS_FLAGS: FileSystemFlags;
    const OWNER: *mut bindings::module = ptr::null_mut();

    fn mount(
        fs_type: &'_ mut FileSystemType,
        flags: c_int,
        device_name: &CStr,
        data: Option<&mut Self::MountOptions>,
    ) -> Result<*mut bindings::dentry>;

    fn kill_super(sb: &mut SuperBlock);

    fn fill_super(
        sb: &mut SuperBlock,
        data: Option<&mut Self::MountOptions>,
        silent: c_int,
    ) -> Result;
}

pub trait DeclaredFileSystemType: FileSystemBase {
    fn file_system_type() -> *mut bindings::file_system_type;
}

#[macro_export]
macro_rules! declare_fs_type {
    ($T:ty, $S:ident) => {
        static mut $S: $crate::bindings::file_system_type = $crate::bindings::file_system_type {
            name: <$T as $crate::fs::FileSystemBase>::NAME.as_char_ptr() as *const _,
            fs_flags: <$T as $crate::fs::FileSystemBase>::FS_FLAGS.into_int(),
            owner: <$T as $crate::fs::FileSystemBase>::OWNER,
            mount: Some($crate::fs::mount_callback::<$T>),
            kill_sb: Some($crate::fs::kill_superblock_callback::<$T>),
            ..$crate::fs::DEFAULT_FS_TYPE
        };
        impl $crate::fs::DeclaredFileSystemType for $T {
            fn file_system_type() -> *mut $crate::bindings::file_system_type {
                unsafe { &mut $S as *mut _ }
            }
        }
    };
}

pub unsafe extern "C" fn mount_callback<T: FileSystemBase>(
    fs_type: *mut bindings::file_system_type,
    flags: c_int,
    device_name: *const c_char,
    data: *mut c_void,
) -> *mut bindings::dentry {
    unsafe {
        let fs_type = &mut *fs_type;
        let device_name = CStr::from_char_ptr(device_name);
        let data = (data as *mut T::MountOptions).as_mut();
        ret_err_ptr!(T::mount(fs_type, flags, device_name, data))
    }
}

pub unsafe extern "C" fn kill_superblock_callback<T: FileSystemBase>(
    sb: *mut bindings::super_block,
) {
    unsafe {
        let sb = sb
            .as_mut()
            .expectk("kill_superblock got NULL super block")
            .as_mut();
        T::kill_super(sb);
    }
}

pub const DEFAULT_FS_TYPE: bindings::file_system_type = bindings::file_system_type {
    name: ptr::null(),
    fs_flags: 0,
    init_fs_context: None,
    parameters: ptr::null(),
    mount: None,
    kill_sb: None,
    owner: ptr::null_mut(),
    next: ptr::null_mut(),
    fs_supers: bindings::hlist_head {
        first: ptr::null_mut(),
    },
    s_lock_key: bindings::lock_class_key {},
    s_umount_key: bindings::lock_class_key {},
    s_vfs_rename_key: bindings::lock_class_key {},
    s_writers_key: [bindings::lock_class_key {}; 3],
    i_lock_key: bindings::lock_class_key {},
    i_mutex_key: bindings::lock_class_key {},
    i_mutex_dir_key: bindings::lock_class_key {},
};
