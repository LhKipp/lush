use crate::cmd_prelude::*;
pub use parking_lot::RwLock;
pub use std::sync::Arc;

pub(crate) trait LuNativeStdMod: Send + Sync {
    fn id(&self) -> String;
    fn src(&self) -> &str;

    fn frame(&self) -> ScopeFrame<Variable> {
        ModInfo::module_from_src(self.src().into(), ModPath::StdPath(self.id()))
            .expect("Std Module never raises error")
    }
}

pub(crate) trait LuRustStdMod: Send + Sync {
    fn id(&self) -> String;
    fn rust_decl(&self) -> SourceCodeItem;
    fn uses(&self) -> Vec<ModPath>;
    fn uses_as_use_path(&self) -> Vec<UsePath> {
        let decl = self.rust_decl();
        self.uses()
            .into_iter()
            .map(|mod_path| UsePath::new(mod_path, decl.clone()))
            .collect()
    }
    fn cmds(&self) -> Vec<Rc<dyn Command>>;
    fn strcts(&self) -> Vec<Arc<RwLock<Strct>>>;
    fn frame(&self) -> ScopeFrame<Variable> {
        let self_mod_path = ModPath::StdPath(self.id());
        let modi = ModInfo::new_std_module(
            self_mod_path,
            "".to_string().into(),
            self.uses_as_use_path(),
        );

        let mut frame = ScopeFrame::new(ScopeFrameTag::ModuleFrame(modi));

        for cmd in self.cmds() {
            frame.insert_var(Variable::new_func(cmd));
        }
        for strct in self.strcts() {
            frame.insert_var(Variable::new_strct_decl_arc(strct));
        }

        frame
    }
}

#[allow(dead_code)]
pub(crate) enum LuStdMod {
    Native(Box<dyn LuNativeStdMod>),
    Rust(Box<dyn LuRustStdMod>),
}

impl LuStdMod {
    pub fn id(&self) -> String {
        match self {
            LuStdMod::Native(m) => m.id(),
            LuStdMod::Rust(m) => m.id(),
        }
    }
    pub fn frame(&self) -> ScopeFrame<Variable> {
        match self {
            LuStdMod::Native(m) => m.frame(),
            LuStdMod::Rust(m) => m.frame(),
        }
    }
}
