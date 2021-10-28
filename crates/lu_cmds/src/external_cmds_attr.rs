use std::collections::HashMap;

use lu_error::{lu_source_code_item, SourceCodeItem};
use lu_interpreter_structs::{CmdAttribute, CmdAttributeVariant::*};
use once_cell::sync::Lazy;

pub(crate) static EXT_CMDS_DEF_ATTRIBUTES: Lazy<Vec<CmdAttribute>> =
    Lazy::new(|| vec![(PurityUnknown, lu_source_code_item!()).into()]);

pub(crate) static EXT_CMDS_ATTRIBUTES: Lazy<HashMap<&str, Vec<CmdAttribute>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("awk", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("bc", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("cat", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("col", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("comm", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("cut", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("date", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("diff", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("echo", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("fmt", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("grep", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("groff", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("gzip", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("gunzip", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("head", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("iconv", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("jobs", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("nl", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("pandoc", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("paste", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("pr", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("ps", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("readelf", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("sed", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("seq", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("sha256sum", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("shuf", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("sort", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("tac", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("tail", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("tee", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("tr", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("uniq", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("wc", vec![(Pure, lu_source_code_item!()).into()]);
    m.insert("xargs", vec![(Pure, lu_source_code_item!()).into()]);
    m
});
