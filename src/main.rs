use std::env;

use clap::App;
use lu_cli::start_cli;
use lu_cmds::builtin;
use lu_error::lu_source_code_item;
use lu_interpreter::{Interpreter, InterpreterCfg};
use lu_interpreter_structs::*;
use lu_stdx::new_amtx;
use lu_text_util::SourceCode;

fn main() {
    std::process::exit(ret_code_main())
}

fn ret_code_main() -> i32 {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .is_test(false)
        .try_init()
        .unwrap();

    // Create a home for the homeless :)
    if let Err(e) = lu_cfg_home::init_home_dir() {
        println!("{:?}", e);
        println!("Aborting because of error");
        return 1;
    }

    let arg_matches = App::new("lush")
        .version("0.1")
        .author("Leonhard Kipp. <leonhard.kipp@alumni.fh-aachen.de>")
        .about("Lu-Shell Interpreter")
        .args_from_usage(
            "--debug      'Runs in debug mode'
            [FILE]      'File to run. If no file is provided a REPL is started'",
        )
        .get_matches();

    let mut global_frame = make_global_frame();

    if arg_matches.is_present("debug") {
        // It is a debug session
        set_new_dbg_session(&mut global_frame);
    }

    if let Some(file_to_run) = arg_matches.value_of("FILE") {
        let code = match SourceCode::new_file(file_to_run.into()) {
            Ok(code) => code,
            Err(e) => {
                // TODO make LuErr display
                eprintln!("Could not read FILE argument. {:?}", e);
                return 1;
            }
        };

        let intprt_config = match InterpreterCfg::try_default() {
            Ok(cfg) => cfg,
            Err(e) => {
                // TODO better formating of e. But its so unusual
                println!("Error while generating default interpreter: {:?}", e);
                return 1;
            }
        };

        let (scope, errs) = Interpreter::ty_check(code, global_frame, &intprt_config).split();
        if !errs.is_empty() {
            if let Err(e) = lu_error_reporting::report_to_term(&errs, &scope) {
                eprintln!("Ups: An error happend, while printing errors: {}", e)
            }
            return 1;
        }
        let mut scope = new_amtx(scope);
        match Interpreter::eval(&mut scope) {
            Ok(_) => {
                // TODO v should be deserialized and passed to the parent lu-shell (if any)
                // maybe pass via flag?
            }
            Err(err) => {
                if let Err(e) = lu_error_reporting::report_to_term(&[err], &scope.lock()) {
                    eprintln!("Ups: An error happend, while printing errors: {}", e)
                }
                return 2;
            }
        };
        0
    } else {
        start_cli(global_frame);
        0
    }
}

fn make_global_frame() -> ScopeFrame<Variable> {
    let mut frame = ScopeFrame::new(ScopeFrameTag::GlobalFrame);
    //insert env vars
    for (key, value) in env::vars() {
        frame.insert_var(Variable::new(
            key,
            value.into(),
            lu_source_code_item!().into(),
        ));
    }

    // insert builtin cmds
    for cmd in builtin::all_builtin_cmds() {
        frame.insert_var(Variable::new_func(cmd));
    }
    frame
}
