use lu_error::LuResult;
use std::path::PathBuf;
use xdg::BaseDirectories;

pub fn get_xdg_base_dir() -> LuResult<BaseDirectories> {
    BaseDirectories::with_prefix("lush").map_err(|_| todo!())
}

pub fn cfg_home() -> LuResult<PathBuf> {
    get_xdg_base_dir().map(|base_dir| base_dir.get_config_home())
}

pub fn dbg_history() -> LuResult<PathBuf> {
    cfg_home().map(|mut home| {
        home.push("dbg_history");
        home
    })
}
