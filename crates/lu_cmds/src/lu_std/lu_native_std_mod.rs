use std::rc::Rc;

use lu_error::SourceCodeItem;
use lu_interpreter_structs::{
    Command, ModInfo, ModPath, ModPathVariant, ScopeFrame, ScopeFrameTag, UsePath, Variable,
};
use lu_text_util::SourceCode;

pub(crate) trait LuNativeStdMod: Send + Sync {
    fn id(&self) -> Vec<String>;
    fn src(&self) -> &str;

    fn frame(&self) -> ScopeFrame<Variable> {
        ModInfo::module_from_src(
            self.src().into(),
            ModPath::new(self.id(), ModPathVariant::StdPath),
        )
        .expect("Std Module never raises error")
    }
}

pub(crate) trait LuRustStdMod: Send + Sync {
    fn id(&self) -> Vec<String>;
    fn rust_decl(&self) -> SourceCodeItem;
    fn rust_src(&self) -> SourceCode;
    fn uses(&self) -> Vec<ModPath>;
    fn uses_as_use_path(&self) -> Vec<UsePath> {
        let decl = self.rust_decl();
        self.uses()
            .into_iter()
            .map(|mod_path| UsePath::new(mod_path, decl.clone()))
            .collect()
    }
    fn cmds(&self) -> Vec<Rc<dyn Command>>;
    fn frame(&self) -> ScopeFrame<Variable> {
        let self_mod_path = ModPath::new(self.id(), ModPathVariant::StdPath);
        let modi = ModInfo::new_std_module(self_mod_path, self.rust_src(), self.uses_as_use_path());

        let mut frame = ScopeFrame::new(ScopeFrameTag::ModuleFrame(modi));

        for cmd in self.cmds() {
            frame.insert_var(Variable::new_func(cmd));
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
    pub fn id(&self) -> Vec<String> {
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
