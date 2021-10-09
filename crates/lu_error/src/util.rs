use std::iter::FromIterator;

use crate::LuErr;

pub struct Outcome<T> {
    pub val: T,
    pub errs: Vec<LuErr>,
}

impl<T> Outcome<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Outcome<U> {
        Outcome {
            val: f(self.val),
            errs: self.errs,
        }
    }
    pub fn split(self) -> (T, Vec<LuErr>) {
        (self.val, self.errs)
    }
    pub fn new(value: T, errors: Vec<LuErr>) -> Self {
        Self {
            val: value,
            errs: errors,
        }
    }
    pub fn ok(val: T) -> Self {
        Outcome::new(val, vec![])
    }

    pub fn from_result(result: Result<T, LuErr>, default: T) -> Self {
        match result {
            Ok(v) => Outcome::ok(v),
            Err(e) => Outcome::new(default, vec![e]),
        }
    }
}

// TODO  From<T> for Outcome would be better...
impl<T> From<(T, Vec<LuErr>)> for Outcome<T> {
    fn from((val, errs): (T, Vec<LuErr>)) -> Self {
        Outcome::new(val, errs)
    }
}

impl<T> FromIterator<Outcome<T>> for Outcome<Vec<T>> {
    fn from_iter<O: IntoIterator<Item = Outcome<T>>>(iter: O) -> Self {
        let mut values = Vec::new();
        let mut errors = Vec::new();

        for outcome in iter {
            values.push(outcome.val);
            errors.extend(outcome.errs);
        }

        Outcome {
            val: values,
            errs: errors,
        }
    }
}
