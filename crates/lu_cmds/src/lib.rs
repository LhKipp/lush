mod print;
mod test_print;

pub use print::PrintCmd;
pub use test_print::TestPrintCmd;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
