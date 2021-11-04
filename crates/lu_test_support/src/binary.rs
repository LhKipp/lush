use std::process::{Command, Stdio};

fn get_lush_binary_path() -> &'static str {
    path!("../../target/debug/lush")
}

pub fn run_binary(args: &[&str]) -> (i32, String, String) {
    // panic!("{}", get_lush_binary_path());
    let process = match Command::new(get_lush_binary_path())
        .args(args)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(why) => panic!("Can't run test {}", why.to_string()),
    };

    let output = process.wait_with_output().unwrap();
    (
        output.status.code().unwrap(),
        std::str::from_utf8(&output.stdout).unwrap().to_string(),
        std::str::from_utf8(&output.stderr).unwrap().to_string(),
    )
}
