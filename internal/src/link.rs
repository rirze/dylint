use crate::{library_filename_with_toolchain, parse_plain_path};
use anyhow::{Context, Result, anyhow};
use std::{
    ffi::OsStr,
    fs::copy,
    path::{Path, PathBuf},
};

pub fn copy_library(plain_path: &Path, lib_name: &str, toolchain: &str) -> Result<()> {
    assert_eq!(
        parse_plain_path(plain_path).as_deref(),
        Some(lib_name),
        "`plain_path` ({}) and `lib_name` ({}) do not correspond",
        plain_path.display(),
        lib_name
    );
    let filename_with_toolchain = library_filename_with_toolchain(lib_name, toolchain);
    let parent = plain_path
        .parent()
        .ok_or_else(|| anyhow!("Could not get parent directory"))?;
    let path_with_toolchain = strip_deps(parent).join(filename_with_toolchain);
    copy(plain_path, &path_with_toolchain).with_context(|| {
        format!(
            "Could not copy `{}` to `{}`",
            plain_path.to_string_lossy(),
            path_with_toolchain.to_string_lossy()
        )
    })?;

    Ok(())
}

fn strip_deps(path: &Path) -> PathBuf {
    if path.file_name() == Some(OsStr::new("deps")) {
        path.parent()
    } else {
        None
    }
    .unwrap_or(path)
    .to_path_buf()
}
