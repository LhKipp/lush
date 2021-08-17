use lu_error::{FsErr, LuResult};
use std::{fs, path::Path};

pub fn read_to_string<P: AsRef<Path>>(path: P) -> LuResult<String> {
    fs::read_to_string(path.as_ref())
        .map_err(|e| FsErr::ReadFailed(path.as_ref().into(), e.to_string()))
        .map_err(|e| e.into())
}

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> LuResult<()> {
    fs::write(path.as_ref(), contents)
        .map_err(|e| FsErr::WriteFailed(path.as_ref().into(), e.to_string()))
        .map_err(|e| e.into())
}
