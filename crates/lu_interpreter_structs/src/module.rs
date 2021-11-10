use crate::{
    Command, Function, ModPath, ScopeFrame, ScopeFrameTag, Strct, StrctField, UsePath, Variable,
};
use itertools::Itertools;
use log::debug;
use lu_error::util::Outcome;
use lu_syntax::{
    ast::{self, SourceFileNode, StrctStmtNode, UseStmtNode},
    AstNode, Parse,
};
use lu_text_util::SourceCode;
use std::{convert::TryInto, fmt::Display};

#[derive(Clone, Debug, Eq)]
pub struct ModInfo {
    pub id: ModPath,
    pub src: SourceCode,
    /// Some for regular modules, None for rust-std-modules
    /// None for Cli module (must be recomputed if needed)
    pub node: Option<SourceFileNode>, // TODO
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

    pub fn update_cli_module_info(&mut self, mut other: ModInfo) {
        assert!(self.node.is_none());
        assert!(self.id == other.id);
        assert!(self.id.as_interactive().is_some());
        self.src.text.push_str(&other.src.text);
        self.use_paths.append(&mut other.use_paths);
    }

    pub fn mod_int_address(&self) -> Option<usize> {
        match self.id {
            ModPath::PlugPath(_) | ModPath::FilePath(_) => Some(ast::addr_of_node(
                self.node.as_ref().unwrap().syntax().clone(),
            )),
            ModPath::Interactive => Some(ast::CLI_LINE_NODE_ADDRESS),
            ModPath::StdPath(_) => {
                if let Some(sf_node) = &self.node {
                    Some(ast::addr_of_node(sf_node.syntax().clone()))
                } else {
                    None
                }
            }
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
    ///
    /// The mod_id does not necessarily have to correspond to the src.path. However if it does,
    /// one can use module_from_file_src
    pub fn module_from_src(src: SourceCode, mod_id: ModPath) -> Outcome<ScopeFrame<Variable>> {
        let parse = Parse::source_file(src);
        parse.map_flattened(|parse| Self::module_from_parse(parse, mod_id))
    }

    pub fn module_from_file_src(id: ModPath, src: SourceCode) -> Outcome<ScopeFrame<Variable>> {
        Self::module_from_src(src, id)
    }

    pub fn module_from_parse(parse: Parse, mod_id: ModPath) -> Outcome<ScopeFrame<Variable>> {
        Self::module_from_sf_node(mod_id, parse.sf_node, parse.source)
    }

    /// Convert a SourceFileNode to a ScopeFrame representation.
    /// No struct-types will be resolved (they are left as ValueType::StructName)
    pub fn module_from_sf_node(
        mod_id: ModPath,
        source_node: SourceFileNode,
        src: SourceCode,
    ) -> Outcome<ScopeFrame<Variable>> {
        debug!("Converting given sf_node {} to frame", mod_id);
        let sourced_file = Outcome::ok(Self::source_structures_from(&source_node, mod_id.clone()));
        sourced_file.map(|sourced_file| {
            let mut frame = ScopeFrame::new(ScopeFrameTag::ModuleFrame(Self {
                id: mod_id,
                src,
                node: Some(source_node),
                use_paths: sourced_file.use_paths,
            }));

            for (_, funcs) in &sourced_file
                .funcs
                .into_iter()
                .group_by(|func| func.name.clone())
            {
                let funcs: Vec<_> = funcs.map(|f| f.rced()).collect();
                if funcs.len() == 1 {
                    frame.insert_var(Variable::new_func(funcs.into_iter().next().unwrap()));
                } else {
                    frame.insert_var(Variable::new_func_collection(funcs));
                }
            }
            for strct in sourced_file.strcts {
                frame.insert_var(Variable::new_strct_decl(strct));
            }
            frame
        })
    }

    fn source_structures_from(
        source_node: &SourceFileNode,
        source_node_id: ModPath,
    ) -> SourcedFile {
        let block = source_node.block();

        // TODO source variables
        let use_paths = block
            .use_stmts()
            .map(|use_stmt| Self::source_use_stmt(&use_stmt))
            .collect();
        let funcs = block
            .fn_stmts()
            .map(|fn_stmt| Function::from_node(&fn_stmt, source_node_id.clone()))
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

    fn source_struct_stmt(struct_stmt: &StrctStmtNode) -> Strct {
        let name = struct_stmt.name().unwrap_or("".to_string());

        // Source the struct fields (either user provided or default)
        let fields: Vec<StrctField> = struct_stmt
            .fields()
            .enumerate()
            .map(|(i, field)| {
                StrctField::from_node(
                    &field,
                    i.try_into()
                        .expect("No strct has more than 2 billion fields"),
                )
            })
            .collect();

        Strct::new(name, fields, struct_stmt.to_item())
    }

    fn source_use_stmt(use_stmt: &UseStmtNode) -> UsePath {
        UsePath::from_node(use_stmt)
    }
}

struct SourcedFile {
    strcts: Vec<Strct>,
    funcs: Vec<Function>,
    use_paths: Vec<UsePath>,
}
