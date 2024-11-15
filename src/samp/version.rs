use winapi::shared::minwindef::LPVOID;
use winapi::um::winver::{GetFileVersionInfoA, GetFileVersionInfoSizeA, VerQueryValueA};

use std::ffi::CString;

use crate::utils::FixedFileInfo;

static mut VERSION: Version = Version::Unknown;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum Version {
    V037,
    V037R2, // also unsupported
    V037R3,
    V037R5,
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub struct VersionTest {
    pub major: u16,
    pub minor: u16,
    pub rc: u16,
}

pub fn version() -> Version {
    unsafe {
        if VERSION == Version::Unknown {
            let someshit = CString::new("\\").unwrap();
            let filename = CString::new("samp.dll").unwrap();
            let filename_ptr = filename.as_ptr() as *const i8;
            let size = GetFileVersionInfoSizeA(filename_ptr, 0 as *mut _);

            if size == 0 {
                return Version::Unknown;
            }

            let mut buffer = vec![0u8; size as usize];

            let buffer_ptr = buffer.as_mut_ptr() as *mut _;

            if GetFileVersionInfoA(filename_ptr, 0, size, buffer_ptr) == 0 {
                return Version::Unknown;
            }

            let mut fileinfo_ptr: usize = 0; // pointer
            let mut length = 0;

            if VerQueryValueA(
                buffer_ptr,
                someshit.as_ptr() as *const _,
                &mut *((&mut fileinfo_ptr) as *mut _ as *mut LPVOID),
                &mut length,
            ) == 0
            {
                return Version::Unknown;
            }

            let fileinfo = &*(fileinfo_ptr as *const FixedFileInfo);

            let major = fileinfo.file_version_ms & 0xFF;
            let minor = fileinfo.file_version_ls >> 16 & 0xFF;
            let rc = fileinfo.file_version_ls & 0xFF;

            let version = match (major, minor, rc) {
                (3, 7, 0) => Version::V037,
                (3, 7, 2) => Version::V037R3,
                _ => Version::V037R5, //TODO("Change version")
            };

            VERSION = version;

            version
        } else {
            VERSION
        }
    }
}
pub fn get_version() -> VersionTest {
    unsafe {
        let someshit = CString::new("\\").unwrap();
        let filename = CString::new("samp.dll").unwrap();
        let filename_ptr = filename.as_ptr() as *const i8;
        let size = GetFileVersionInfoSizeA(filename_ptr, 0 as *mut _);

        if size == 0 {
            return VersionTest { major: 0, minor: 0, rc: 0 };
        }

        let mut buffer = vec![0u8; size as usize];
        let buffer_ptr = buffer.as_mut_ptr() as *mut _;

        if GetFileVersionInfoA(filename_ptr, 0, size, buffer_ptr) == 0 {
            return VersionTest { major: 0, minor: 0, rc: 0 };
        }

        let mut fileinfo_ptr: usize = 0;
        let mut length = 0;

        if VerQueryValueA(
            buffer_ptr,
            someshit.as_ptr() as *const _,
            &mut *((&mut fileinfo_ptr) as *mut _ as *mut LPVOID),
            &mut length,
        ) == 0 {
            return VersionTest { major: 0, minor: 0, rc: 0 };
        }

        let fileinfo = &*(fileinfo_ptr as *const FixedFileInfo);

        let major = (fileinfo.file_version_ms >> 16) as u16;
        let minor = (fileinfo.file_version_ms & 0xFFFF) as u16;
        let rc = (fileinfo.file_version_ls & 0xFFFF) as u16;

        VersionTest { major, minor, rc }
    }
}



pub fn is_unknown_version() -> bool {
    version() == Version::Unknown
}
