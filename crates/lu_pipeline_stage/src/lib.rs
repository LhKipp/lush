use lu_error::LuErr;

pub trait PipelineStage {
    fn get_prev_stage(&self) -> Option<&dyn PipelineStage>;
}

pub trait ErrorContainer: PipelineStage {
    fn get_mut_errors(&mut self) -> &mut Vec<LuErr>;
    fn get_errors(&self) -> &Vec<LuErr>;

    fn push_err(&mut self, e: LuErr) {
        self.get_mut_errors().push(e)
    }

    fn ok_or_record<T>(&mut self, res: Result<T, LuErr>) -> Option<T> {
        match res {
            Ok(t) => Some(t),
            Err(e) => {
                self.push_err(e);
                None
            }
        }
    }
}

// pub trait RecordResult {
//     fn record<T>(self, container: &mut dyn ErrorContainer) -> Option<T>;
// }

// impl<T, E> RecordResult for Result<T, E> {
//     fn record<F>(self, container: &mut dyn ErrorContainer) -> Option<F> {
//         container.ok_or_record(self)
//     }
// }
