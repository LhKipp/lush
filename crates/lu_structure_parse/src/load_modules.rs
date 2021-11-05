use std::{collections::HashSet, mem::replace};

use log::debug;
use lu_error::util::Outcome;
use lu_interpreter_structs::{ModInfo, ModPath, ScopeFrame, Variable};
use lu_text_util::SourceCode;
use walkdir::WalkDir;

use crate::LoadModulesConfig;

/// Load all modules required by the given ScopeFrame, and the modules required by the
/// included modules, and the modules required by these, ... (recursive)
pub fn load_mod_paths(
    start_frame: ScopeFrame<Variable>,
    cfg: &LoadModulesConfig,
) -> Outcome<Vec<ScopeFrame<Variable>>> {
    debug!("loading all modules required by a (start)-frame (recursive)");
    let mut all_frames = vec![start_frame];
    let mut errs = vec![];

    let modi = all_frames[0]
        .get_tag()
        .clone()
        .into_module_frame()
        .expect("Arg must be SourceFileFrame");

    let mut sourced_modules = HashSet::new();
    sourced_modules.insert(modi.id);

    let mut paths_to_source = modi.use_paths;

    loop {
        // replace to mutate paths_to_source in for loop
        for use_path in replace(&mut paths_to_source, vec![]) {
            if sourced_modules.contains(&use_path.mod_path) {
                continue;
            }
            debug!("Loading module: {}", use_path);
            match &use_path.mod_path {
                ModPath::StdPath(_) => {
                    let frame = (cfg.load_std_module_func)(&use_path.mod_path)[0].clone();
                    let modi = frame.get_mod_tag();
                    paths_to_source.extend(modi.use_paths.clone());
                    all_frames.push(frame);
                }
                ModPath::PlugPath(plug_f_path) => {
                    let plug_f_path = cfg.plugin_dir.join(plug_f_path);

                    debug!("Loading plug-mod: {:?}", plug_f_path);
                    for f_entry in WalkDir::new(plug_f_path).into_iter() {
                        let file = f_entry.expect("TODO").into_path();
                        debug!("Loading mod: {:?}", file);
                        match SourceCode::new_file(file.clone()) {
                            Err(e) => {
                                debug!("Loading of plug-mod with f_path: {:?} failed", file);
                                errs.push(e);
                            }
                            Ok(source_code) => {
                                let (module, new_mod_err) =
                                    ModInfo::module_from_file_src(source_code, cfg.plugin_dir)
                                        .split();
                                let modi = module.get_mod_tag();
                                paths_to_source.extend(modi.use_paths.clone());
                                errs.extend(new_mod_err);
                                all_frames.push(module);
                            }
                        }
                    }
                }
                ModPath::FilePath(_) => {
                    todo!("Impl sourcing of files")
                }
            }
            sourced_modules.insert(use_path.mod_path);
        }
        if paths_to_source.is_empty() {
            break; // No more work
        }
    }

    Outcome::ok(all_frames)
}
