use crate::{
    Command, Function, ModPath, ModPathVariant, ScopeFrame, ScopeFrameTag, Signature, Strct,
    StrctField, UsePath, Variable,
};
use lu_error::util::Outcome;
use lu_parser::grammar::SourceFileRule;
use lu_syntax::{
    ast::{FnStmtNode, SourceFileNode, StrctStmtNode, UseStmtNode},
    AstNode, Parse,
};
use lu_text_util::SourceCode;
use std::{fmt::Display, path::Path};

#[derive(Clone, Debug, Eq)]
pub struct ModInfo {
    pub id: ModPath,
    pub src: SourceCode,
    /// Some for regular modules, None for rust-std-modules
    pub node: Option<SourceFileNode>,
    pub use_paths: Vec<UsePath>,
}

impl ModInfo {
    pub fn new_std_module(id: ModPath, src: SourceCode, use_paths: Vec<UsePath>) -> ModInfo {
        ModInfo {
            id,
            src,
            node: None,
            use_paths,
        }
    }
}

impl Display for ModInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl PartialEq for ModInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl ModInfo {
    /// Convert a SourceFileNode to a ScopeFrame representation.
    /// No struct-types will be resolved (they are left as ValueType::StructName)
    pub fn module_from_src(src: SourceCode, plugin_dir: &Path) -> Outcome<ScopeFrame<Variable>> {
        let parse = Parse::rule(src, &SourceFileRule {});
        parse.map_flattened(|parse| Self::module_from_parse(parse, plugin_dir))
    }

    pub fn module_from_parse(parse: Parse, plugin_dir: &Path) -> Outcome<ScopeFrame<Variable>> {
        assert!(parse.is_sf_parse());
        let self_mod_path = ModPath::from_src_code(&parse.source, plugin_dir);
        Self::module_from_sf_node(self_mod_path, parse.source_file_node(), parse.source)
    }

    /// Convert a SourceFileNode to a ScopeFrame representation.
    /// No struct-types will be resolved (they are left as ValueType::StructName)
    pub fn module_from_sf_node(
        mod_id: ModPath,
        source_node: SourceFileNode,
        src: SourceCode,
    ) -> Outcome<ScopeFrame<Variable>> {
        let sourced_file = Outcome::ok(Self::source_structures_from(&source_node, mod_id.clone()));
        sourced_file.map(|sourced_file| {
            let mut frame = ScopeFrame::new(ScopeFrameTag::ModuleFrame(Self {
                id: mod_id,
                src,
                node: Some(source_node),
                use_paths: sourced_file.use_paths,
            }));

            for func in sourced_file.funcs {
                frame.insert_var(Variable::new_func(func.rced()));
            }
            for strct in sourced_file.strcts {
                frame.insert_var(Variable::new_strct(strct));
            }
            frame
        })
    }

    fn source_structures_from(
        source_node: &SourceFileNode,
        source_node_id: ModPath,
    ) -> SourcedFile {
        let block = source_node.block().unwrap();

        // TODO source variables
        let use_paths = block
            .use_stmts()
            .map(|use_stmt| Self::source_use_stmt(&use_stmt))
            .collect();
        let funcs = block
            .fn_stmts()
            .map(|fn_stmt| Self::source_fn_stmt(&fn_stmt, source_node_id.clone()))
            .collect();
        let strcts = block
            .struct_stmts()
            .map(|strct_stmt| Self::source_struct_stmt(&strct_stmt))
            .collect();

        SourcedFile {
            strcts,
            funcs,
            use_paths,
        }
    }

    fn source_fn_stmt(fn_stmt: &FnStmtNode, source_file_id: ModPath) -> Function {
        let name = fn_stmt.name().unwrap_or("".to_string());
        // Source the signature (either user provided or default)
        let sign = Signature::from_sign_and_stmt(fn_stmt.signature(), fn_stmt.decl_item());

        Function::new(name, sign, fn_stmt.clone(), source_file_id)
    }

    fn source_struct_stmt(struct_stmt: &StrctStmtNode) -> Strct {
        let name = struct_stmt.name().unwrap_or("".to_string());

        // Source the struct fields (either user provided or default)
        let fields: Vec<StrctField> = struct_stmt
            .fields()
            .map(|field| StrctField::from_node(&field))
            .collect();

        Strct::new(name, fields, struct_stmt.to_item())
    }

    fn source_use_stmt(use_stmt: &UseStmtNode) -> UsePath {
        let ty = if use_stmt.is_std_path() {
            ModPathVariant::StdPath
        } else if use_stmt.is_plugin_path() {
            ModPathVariant::PluginPath
        } else {
            assert!(use_stmt.is_file_path());
            ModPathVariant::FilePath
        };
        UsePath::new(ModPath::new(use_stmt.parts(), ty), use_stmt.to_item())
    }
}

struct SourcedFile {
    strcts: Vec<Strct>,
    funcs: Vec<Function>,
    use_paths: Vec<UsePath>,
}
