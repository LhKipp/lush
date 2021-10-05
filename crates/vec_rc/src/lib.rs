#[macro_export]
macro_rules! vec_rc {
    ($elem:expr; $n:expr) => (vec![Rc::new($elem); $n]);
    ($($x:expr),*) => (vec![$(Rc::new($x)),*]);
    ($($x:expr,)*) => (vec_rc![$($x),*]);
}
