use serde::{Deserialize, Serialize};
use std::iter::FromIterator;

use crate::{LuErr, LuResult, LuResults};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Outcome<T> {
    pub val: T,
    pub errs: Vec<LuErr>,
}

impl<T> Outcome<T> {
    /// Map and accumulate errors
    pub fn map_flattened<U, F: FnOnce(T) -> Outcome<U>>(self, f: F) -> Outcome<U> {
        self.map(f).flatten()
    }

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

    pub fn unwrap(self) -> T {
        assert!(self.errs.is_empty());
        self.val
    }

    pub fn expect(self, msg: &'static str) -> T {
        assert!(self.errs.is_empty(), "{}. Errors: {:?}", msg, self.errs);
        self.val
    }
    pub fn as_results(self) -> LuResults<T> {
        if self.errs.is_empty() {
            Ok(self.val)
        } else {
            Err(self.errs)
        }
    }
}

impl<T> Outcome<Outcome<T>> {
    pub fn flatten(mut self) -> Outcome<T> {
        let (val, inner_errs) = self.val.split();
        self.errs.extend(inner_errs);
        Outcome::new(val, self.errs)
    }
}

// TODO  From<T> for Outcome would be better...
impl<T> From<T> for Outcome<T> {
    fn from(val: T) -> Self {
        Outcome::ok(val)
    }
}

impl<T> Into<LuResults<T>> for Outcome<T> {
    fn into(self) -> LuResults<T> {
        self.as_results()
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

impl<T> FromIterator<LuResult<T>> for Outcome<Vec<T>> {
    fn from_iter<O: IntoIterator<Item = LuResult<T>>>(iter: O) -> Self {
        let mut values = Vec::new();
        let mut errors = Vec::new();

        for result in iter {
            match result {
                Ok(v) => values.push(v),
                Err(e) => errors.push(e),
            }
        }

        Outcome {
            val: values,
            errs: errors,
        }
    }
}
