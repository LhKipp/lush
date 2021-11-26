use std::{collections::HashSet, mem::replace};

use log::debug;
use lu_error::{util::Outcome, AstErr};
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

    let start_file_path = match &modi.id {
        ModPath::PlugPath(p) => Some(cfg.plugin_dir.join(p)),
        ModPath::StdPath(_) => None,
        ModPath::FilePath(p) => Some(p.clone()),
        ModPath::Interactive => Some(cfg.pwd.clone()),
    };

    let mut sourced_modules = HashSet::new();
    sourced_modules.insert(modi.id);

    // To handle the relative use paths correctly, we need to keep track of who (PathBuf) imported them
    let mut paths_to_source: Vec<_> = modi
        .use_paths
        .into_iter()
        .map(|path| (path, start_file_path.clone()))
        .collect();

    loop {
        // replace to mutate paths_to_source in for loop
        for (use_path, includer_path) in replace(&mut paths_to_source, vec![]) {
            if sourced_modules.contains(&use_path.mod_path) {
                continue;
            }
            debug!("Loading module: {}", use_path);
            match &use_path.mod_path {
                ModPath::StdPath(_) => {
                    let (frames, load_std_mod_errs) =
                        (cfg.load_std_module_func)(&use_path.mod_path, &use_path.decl).split();
                    errs.extend(load_std_mod_errs);
                    for frame in frames {
                        let modi = frame.get_mod_tag();
                        paths_to_source
                            // std never includes relative
                            .extend(modi.use_paths.clone().into_iter().map(|path| (path, None)));
                        all_frames.push(frame);
                    }
                }
                ModPath::PlugPath(plug_f_path) => {
                    let plug_f_path = cfg.plugin_dir.join(plug_f_path);

                    debug!("Loading plug-mod: {:?}", plug_f_path);
                    for f_entry in WalkDir::new(&plug_f_path).into_iter() {
                        let file = f_entry.expect("TODO").into_path();
                        debug!("Loading mod: {:?}", file);
                        match SourceCode::new_file(file.clone()) {
                            Err(e) => {
                                debug!("Loading of plug-mod with f_path: {:?} failed", file);
                                errs.push(e);
                            }
                            Ok(source_code) => {
                                let (module, new_mod_err) = ModInfo::module_from_file_src(
                                    use_path.mod_path.clone(),
                                    source_code,
                                )
                                .split();
                                let modi = module.get_mod_tag();
                                paths_to_source.extend(
                                    modi.use_paths
                                        .clone()
                                        .into_iter()
                                        .map(|path| (path, Some(plug_f_path.clone())))
                                        .collect::<Vec<_>>(),
                                );
                                errs.extend(new_mod_err);
                                all_frames.push(module);
                            }
                        }
                    }
                }
                ModPath::FilePath(file) => {
                    let file = if file.is_absolute() {
                        file.clone()
                    } else {
                        // Include path is relative to the includer
                        if let Some(mut includer_path) = includer_path {
                            includer_path.pop();
                            includer_path.push(file);
                            includer_path
                        } else {
                            // Shouldn't realy happen. Include must come from std, (and std
                            // does not include relative). Its most likly a bug in the impl
                            errs.push(AstErr::CantUseRelativeInclude(use_path.decl.clone()).into());
                            continue;
                        }
                    };
                    match SourceCode::new_file(file.clone()) {
                        Err(e) => {
                            debug!("Loading of file-mod with f_path: {:?} failed", file);
                            errs.push(e);
                        }
                        Ok(source_code) => {
                            let (module, new_mod_err) = ModInfo::module_from_file_src(
                                // We fixed up the file path.
                                //     the includer expects a module with UsePath ./include_file.lu
                                // The file (due to relative include fixup), has a modpath of
                                //      includer_file.lu/../include_file.lu
                                // Both thinks need to be the same path, for correct lookup in
                                // scope
                                // We can either: - Change includers include path here (IMO best
                                // option)
                                //                - Or insert a ModInfo with a slightly wrong mod
                                //                path (Will lead to multiple same modules and
                                //                other anomalies. Best if we do the first?) Thats
                                //                done right now
                                use_path.mod_path.clone(),
                                source_code,
                            )
                            .split();
                            let modi = module.get_mod_tag();
                            paths_to_source.extend(
                                modi.use_paths
                                    .clone()
                                    .into_iter()
                                    .map(|path| (path, Some(file.clone())))
                                    .collect::<Vec<_>>(),
                            );
                            errs.extend(new_mod_err);
                            all_frames.push(module);
                        }
                    }
                }
                ModPath::Interactive => unreachable!("Cant import the interactive file"),
            }
            // TODO not correct for multiple relative includes
            sourced_modules.insert(use_path.mod_path);
        }
        if paths_to_source.is_empty() {
            break; // No more work
        }
    }

    Outcome::new(all_frames, errs)
}
