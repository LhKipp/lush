use lu_interpreter::*;
use lu_interpreter_structs::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;

const HIST_FILE: &str = "/home/leo/.config/lu/history.txt";
const PLUGIN_DIR: &str = "/home/leo/.config/lu/plugins";
pub fn start_cli() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history(HIST_FILE).is_err() {
        println!("No previous history.");
    }

    let global_frame = ScopeFrame::new(ScopeFrameTag::GlobalFrame);
    let mut intprt = InteractiveInterpreter::new(
        global_frame,
        InterpreterCfg {
            plugin_dir: PLUGIN_DIR.into(),
        },
    );

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                match intprt.eval_line(&line) {
                    Ok(val) => print!("{}", val),
                    Err(e) => print!("{:?}", e),
                };
                rl.add_history_entry(line.as_str());
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(HIST_FILE).unwrap();
}
