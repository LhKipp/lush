use log::debug;
use lu_interpreter::*;
use lu_interpreter_structs::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn start_cli(global_frame: ScopeFrame<Variable>) {
    let hist_file = lu_cfg_home::cli_history();
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if let Ok(hist_file) = &hist_file {
        if rl.load_history(hist_file).is_err() {
            eprintln!("No previous history.");
        }
    }

    let intprt_cfg = match InterpreterCfg::try_default() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error while generating default interpreter: {:?}", e);
            return;
        }
    };
    let mut intprt = InteractiveInterpreter::new(global_frame, intprt_cfg);

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(mut line) => {
                line.push('\n');
                match intprt.eval_line(&line) {
                    Ok(_) => {
                        // Value will already be printed
                    }
                    Err(err) => {
                        debug!("EvalLine returned errors: {:?}", err);
                        if let Err(e) =
                            lu_error_reporting::report_to_term(&err, &intprt.ty_checker.scope)
                        {
                            eprintln!("Ups: An error happend, while printing errors: {}", e)
                        }
                    }
                };
                line.pop(); // Remove pushed '\n'
                rl.add_history_entry(line.as_str());
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    if let Ok(hist_file) = hist_file {
        if let Err(e) = rl.save_history(&hist_file) {
            eprintln!("Could not save cli_history: {}", e);
        }
    }
}
