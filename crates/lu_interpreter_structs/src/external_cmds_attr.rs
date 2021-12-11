use std::collections::HashMap;

use crate::{CmdAttribute, CmdAttributeVariant::*};
use lu_error::lu_source_code_item;
use once_cell::sync::Lazy;

pub(crate) static EXT_CMDS_DEF_ATTRIBUTES: Lazy<Vec<CmdAttribute>> = Lazy::new(|| {
    vec![
        (PurityUnknown, lu_source_code_item!()).into(),
        (DontParseArguments, lu_source_code_item!()).into(),
    ]
});

pub(crate) static EXT_CMDS_ATTRIBUTES: Lazy<HashMap<&str, Vec<CmdAttribute>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "awk",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "bc",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "cat",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "col",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "comm",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "cut",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "date",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "diff",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "echo",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "fmt",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "grep",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "groff",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "gzip",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "gunzip",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "head",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "iconv",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "jobs",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "nl",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "pandoc",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "paste",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "pr",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "ps",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "readelf",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "sed",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "seq",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "sha256sum",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "shuf",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "sort",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "tac",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "tail",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "tee",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "tr",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "uniq",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "wc",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m.insert(
        "xargs",
        vec![
            (DontParseArguments, lu_source_code_item!()).into(),
            (Pure, lu_source_code_item!()).into(),
        ],
    );
    m
});
