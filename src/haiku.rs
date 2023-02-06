use std::{ffi::OsStr, io, process::Command};

use crate::{CommandExt, IntoResult};

pub fn that<T: AsRef<OsStr>>(path: T) -> io::Result<()> {
    Command::new("/bin/open")
        .arg(path.as_ref())
        .status_without_output()
        .into_result()
}

pub fn with<T: AsRef<OsStr>>(path: T, app: impl Into<String>) -> io::Result<()> {
    Command::new(app.into())
        .arg(path.as_ref())
        .status_without_output()
        .into_result()
}

pub fn with_args<T, I>(path: T, app: impl Into<String>, args: I) -> io::Result<()>
where
    T: AsRef<OsStr>,
    I: IntoIterator<Item = T>,
{
    Command::new(app.into())
        .args::<I, T>(args.into())
        .arg(path.as_ref())
        .status_without_output()
        .into_result()
}
