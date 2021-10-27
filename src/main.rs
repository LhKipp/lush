use clap::App;
use lu_cli::start_cli;
use lu_interpreter::{Interpreter, InterpreterCfg};
use lu_interpreter_structs::*;
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

    let mut global_frame = ScopeFrame::new(ScopeFrameTag::GlobalFrame);

    if arg_matches.is_present("debug") {
        // It is a debug session
        set_dbg_session(&mut global_frame);
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

        let intprt_config = InterpreterCfg::default();
        match Interpreter::new(global_frame, intprt_config).eval(code) {
            Ok(_) => {
                // TODO v should be deserialized and passed to the parent lu-shell (if any)
                // maybe pass via flag?
            }
            Err(errs) => {
                for err in errs {
                    eprintln!("{:?}", err)
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
