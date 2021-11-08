use std::{
    fs::create_dir,
    path::{Path, PathBuf},
};
use temp_testdir::TempDir;

pub struct Playground {
    pub test_dir: TempDir,
}

impl Playground {
    pub fn new() -> Self {
        let self_ = Self {
            test_dir: TempDir::default(),
        };

        create_dir(self_.plugin_dir()).expect("Must succeed");

        self_
    }

    pub fn root(&self) -> &Path {
        self.test_dir.as_ref()
    }

    pub fn plugin_dir(&self) -> PathBuf {
        self.test_dir.join("plugins")
    }

    pub fn permanent(mut self) -> Playground {
        self.test_dir = self.test_dir.permanent();
        self
    }

    pub fn make_file(&self, fname: &str, content: &[u8]) -> PathBuf {
        let path = self.root().join(fname);
        std::fs::write(&path, content).expect("Must work");
        path
    }
}
