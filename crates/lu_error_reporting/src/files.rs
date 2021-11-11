use std::collections::HashMap;

use codespan_reporting::files::{self, Files};
use log::debug;

use crate::lu_source_files_util::{get_file, LU_FILE_ID_MAX, LU_FILE_ID_MIN};

pub(crate) struct DiagnosticFileContainer<'a> {
    file_id_to_content: HashMap<usize, &'a str>,
    file_id_to_name: HashMap<usize, String>,
}

impl<'a> DiagnosticFileContainer<'a> {
    pub fn add_file(&mut self, file_addr: usize, file_name: String, file_content: &'a str) {
        self.file_id_to_content.insert(file_addr, file_content);
        self.file_id_to_name.insert(file_addr, file_name);
    }

    pub fn empty() -> Self {
        Self {
            file_id_to_content: HashMap::new(),
            file_id_to_name: HashMap::new(),
        }
    }
}
impl<'a> Files<'a> for DiagnosticFileContainer<'a> {
    type FileId = usize;

    type Name = String;

    type Source = &'a str;

    fn name(&'a self, id: Self::FileId) -> Result<Self::Name, codespan_reporting::files::Error> {
        match id {
            LU_FILE_ID_MIN..=LU_FILE_ID_MAX => Ok(get_file(id).0.to_string()),
            _ => Ok(self.file_id_to_name.get(&id).unwrap().clone()),
        }
    }

    fn source(
        &'a self,
        id: Self::FileId,
    ) -> Result<Self::Source, codespan_reporting::files::Error> {
        match id {
            LU_FILE_ID_MIN..=LU_FILE_ID_MAX => Ok(get_file(id).1),
            _ => Ok(self.file_id_to_content.get(&id).unwrap()),
        }
    }

    fn line_index(
        &'a self,
        id: Self::FileId,
        byte_index: usize,
    ) -> Result<usize, codespan_reporting::files::Error> {
        let content = self.source(id).unwrap();
        let line_starts: Vec<_> = files::line_starts(content).collect();
        debug!(
            "Found line_starts: {:?} for content\n{}",
            line_starts, content
        );
        let result = match line_starts.binary_search(&byte_index) {
            Ok(line) => Ok(line),
            Err(next_line) => Ok(next_line - 1),
        };
        debug!(
            "Returning line_index {:?} for file:{} byte_index:{}",
            result, id, byte_index
        );
        result
    }

    fn line_range(
        &'a self,
        id: Self::FileId,
        line_index: usize,
    ) -> Result<std::ops::Range<usize>, codespan_reporting::files::Error> {
        let content = self.source(id).unwrap();
        let line_starts: Vec<_> = files::line_starts(content).collect();

        // If line_index and next is present in line_starts
        let line_range = if line_index + 1 < line_starts.len() {
            let line_byte_begin = line_starts[line_index];
            let next_line_byte_begin = line_starts[line_index + 1];
            let line_byte_end = next_line_byte_begin - 1; // \n is 1 byte long
            Ok(line_byte_begin..line_byte_end)
        } else {
            let line_byte_begin = line_starts[line_index];
            Ok(line_byte_begin..(content.as_bytes().len()))
        };
        debug!(
            "Returning line_range {:?} for file:{} line_index:{}",
            line_range, id, line_index
        );
        line_range
    }
}
