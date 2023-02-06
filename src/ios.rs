use std::{ffi::OsStr, io, process::Command};

use crate::{CommandExt, IntoResult};

pub fn that<T: AsRef<OsStr>>(path: T) -> io::Result<()> {
    Command::new("uiopen")
        .arg("--url")
        .arg(path.as_ref())
        .status_without_output()
        .into_result()
}

pub fn with<T: AsRef<OsStr>>(path: T, app: impl Into<String>) -> io::Result<()> {
    Command::new("uiopen")
        .arg("--url")
        .arg(path.as_ref())
        .arg("--bundleid")
        .arg(app.into())
        .status_without_output()
        .into_result()
}

pub fn with_args<T, I>(path: T, app: impl Into<String>, args: I) -> io::Result<()>
where
    T: AsRef<OsStr>,
    I: IntoIterator<Item = T>,
{
    Command::new("uiopen")
        .arg("--url")
        .arg(path.as_ref())
        .arg("--bundleid")
        .arg(app.into())
        .args(args.into())
        .status_without_output()
        .into_result()
}
