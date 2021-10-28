use parking_lot::Mutex;
use std::sync::Arc;

pub type AMtx<T> = Arc<Mutex<T>>;

pub fn new_amtx<T>(val: T) -> AMtx<T> {
    Arc::new(Mutex::new(val))
}
