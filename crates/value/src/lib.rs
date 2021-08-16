pub enum Value {
    Nil,
    Number(f64),
    String(String),
    BareWord(String),
    Array(Vec<Value>),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
