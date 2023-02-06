use std::{
    ffi::{OsStr, OsString},
    io,
    os::windows::ffi::OsStrExt,
    ptr,
};

use std::os::raw::c_int;
use windows_sys::Win32::UI::Shell::ShellExecuteW;
use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOW;

use crate::IntoResult;

fn convert_path(path: &OsStr) -> io::Result<Vec<u16>> {
    let mut quoted_path = OsString::with_capacity(path.len());

    // Surround path with double quotes "" to handle spaces in path.
    quoted_path.push("\"");
    quoted_path.push(&path);
    quoted_path.push("\"");

    let mut wide_chars: Vec<_> = quoted_path.encode_wide().collect();
    if wide_chars.iter().any(|&u| u == 0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path contains NUL byte(s)",
        ));
    }
    wide_chars.push(0);
    Ok(wide_chars)
}

pub fn that<T: AsRef<OsStr>>(path: T) -> io::Result<()> {
    let path = convert_path(path.as_ref())?;
    let operation: Vec<u16> = OsStr::new("open\0").encode_wide().collect();
    let result = unsafe {
        ShellExecuteW(
            0,
            operation.as_ptr(),
            path.as_ptr(),
            ptr::null(),
            ptr::null(),
            SW_SHOW,
        )
    };
    (result as c_int).into_result()
}

pub fn with<T: AsRef<OsStr>>(path: T, app: impl Into<String>) -> io::Result<()> {
    let path = convert_path(path.as_ref())?;
    let operation: Vec<u16> = OsStr::new("open\0").encode_wide().collect();
    let app_name: Vec<u16> = OsStr::new(&format!("{}\0", app.into()))
        .encode_wide()
        .collect();
    let result = unsafe {
        ShellExecuteW(
            0,
            operation.as_ptr(),
            app_name.as_ptr(),
            path.as_ptr(),
            ptr::null(),
            SW_SHOW,
        )
    };
    (result as c_int).into_result()
}

pub fn with_args<T, I>(path: T, app: impl Into<String>, args: I) -> io::Result<()>
where
    T: AsRef<OsStr>,
    I: IntoIterator<Item = T>,
{
    let mut args = convert_path(path.as_ref())?;
    args.pop();
    for arg in args.into() {
        args.push(" ");
        args.push(arg);
    }
    args.push(0);

    let operation: Vec<u16> = OsStr::new("open\0").encode_wide().collect();
    let app_name: Vec<u16> = OsStr::new(&format!("{}\0", app.into()))
        .encode_wide()
        .collect();
    let result = unsafe {
        ShellExecuteW(
            0,
            operation.as_ptr(),
            app_name.as_ptr(),
            args.as_ptr(),
            ptr::null(),
            SW_SHOW,
        )
    };
    (result as c_int).into_result()
}
