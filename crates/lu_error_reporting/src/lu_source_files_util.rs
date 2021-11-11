use crate::lu_source_files::FILE_TO_STR;

pub(crate) const LU_FILE_ID_MIN: usize = 1;
pub(crate) const LU_FILE_ID_MAX: usize = 5000;

// Returns Some(lu_file_addr, content) or None
pub(crate) fn find_file(name: &str) -> Option<(usize, &'static str)> {
    FILE_TO_STR
        .iter()
        .enumerate()
        .find_map(|(i, (f_name, content))| {
            if *f_name == name {
                Some((i + 1, *content)) // + 1 so its greater than CLI_LINE_NODE_ADDRESS
            } else {
                None
            }
        })
}

pub(crate) fn get_file(f_id: usize) -> (&'static str, &'static str) {
    assert!(f_id >= LU_FILE_ID_MIN && f_id <= LU_FILE_ID_MAX);
    FILE_TO_STR[f_id - 1]
}
