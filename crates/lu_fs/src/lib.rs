use lu_error::{FsErr, LuErr, LuResult};
use std::{fs, path::Path};

pub fn read_to_string<P: AsRef<Path>>(path: P) -> LuResult<String> {
    fs::read_to_string(path.as_ref()).map_err(|e| {
        LuErr::FS(FsErr {
            path: path.as_ref().into(),
            source: e.into(),
        })
    })
}

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> LuResult<()> {
    fs::write(path.as_ref(), contents).map_err(|e| {
        LuErr::FS(FsErr {
            path: path.as_ref().into(),
            source: e.into(),
        })
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
