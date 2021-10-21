use lu_cli::start_cli;

fn main() {
    let _ = env_logger::builder()
        .format_timestamp(None)
        .is_test(false)
        .try_init().unwrap();
    start_cli();
}
