use lu_error::SourceCodeItem;

pub const SELECT_CMD_NAME: &str = "select";
pub const SELECT_DEF_STRCT_DECL_ARG_NAME: &str = "LU_SELECT_DEF_STRCT_DECL_ARG";

/// Get the default strct name, for the select cmd stmt with SourceCodeItem
/// `cmd_stmt_decl`
pub fn select_def_strct_name(cmd_stmt_decl: &SourceCodeItem) -> String {
    format!("Select_textrange_{}", cmd_stmt_decl.display_range())
}
