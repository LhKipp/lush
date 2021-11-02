use lu_error::LuResult;
use std::path::PathBuf;
use xdg::BaseDirectories;

pub fn init_home_dir() -> LuResult<()> {
    let home = cfg_home()?;
    if !home.exists() {
        fs_err::create_dir(cfg_home()?)?;
    }
    let files = vec![dbg_history()?];
    for f in files {
        if !f.exists() {
            fs_err::File::create(f)?;
        }
    }
    Ok(())
}

pub fn get_xdg_base_dir() -> LuResult<BaseDirectories> {
    BaseDirectories::with_prefix("lush").map_err(|_| todo!())
}

pub fn cfg_home() -> LuResult<PathBuf> {
    get_xdg_base_dir().map(|base_dir| base_dir.get_config_home())
}

pub fn cli_history() -> LuResult<PathBuf> {
    cfg_home().map(|mut home| {
        home.push("cli_history.txt");
        home
    })
}

pub fn dbg_history() -> LuResult<PathBuf> {
    cfg_home().map(|mut home| {
        home.push("dbg_history");
        home
    })
}

pub fn plugin_dir() -> LuResult<PathBuf> {
    cfg_home().map(|mut home| {
        home.push("plugins");
        home
    })
}
