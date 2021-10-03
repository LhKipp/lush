use log::debug;
use lu_error::LuErr;

pub trait PipelineStage {
    fn get_prev_stage(&self) -> Option<&dyn PipelineStage>;

    fn get_mut_errors(&mut self) -> &mut Vec<LuErr>;
    fn get_errors(&self) -> &Vec<LuErr>;

    /// If all_errors in a stage != get_errors, this func can be overwritten to provide all errors
    fn collect_all_errors_cb(&self) -> Vec<LuErr> {
        self.get_errors().to_vec()
    }

    fn collect_all_errors(&self) -> Vec<LuErr> {
        let mut prev_err = self
            .get_prev_stage()
            .map(PipelineStage::collect_all_errors_cb)
            .unwrap_or_else(Vec::new);
        prev_err.extend(self.get_errors().clone());
        prev_err
    }

    fn push_err(&mut self, e: LuErr) {
        debug!("Recording err: {:?}", e);
        self.get_mut_errors().push(e);
    }

    fn push_errs(&mut self, e: Vec<LuErr>) {
        debug!("Recording errors: {:?}", e);
        self.get_mut_errors().extend(e);
    }

    fn succeeded(&self) -> bool {
        // TODO wrong impl for Resolve (as it can have multiple parses)
        self.get_errors().is_empty()
            && self
                .get_prev_stage()
                .map(PipelineStage::succeeded)
                .unwrap_or(true)
    }

    fn failed(&self) -> bool {
        !self.succeeded()
    }
}

pub trait ErrorContainer: PipelineStage {
    fn ok_or_record<T>(&mut self, res: Result<T, LuErr>) -> Option<T> {
        match res {
            Ok(t) => Some(t),
            Err(e) => {
                self.push_err(e);
                None
            }
        }
    }

    /// Records whether opt was Some (hand has therefore been recorded as an error)
    fn record_option(&mut self, opt: Option<LuErr>) -> bool {
        opt.map(|e| {
            self.push_err(e);
            true
        })
        .unwrap_or(false)
    }
}

impl<Stage: PipelineStage> ErrorContainer for Stage {}

// pub trait RecordResult {
//     fn record<T>(self, container: &mut dyn ErrorContainer) -> Option<T>;
// }

// impl<T, E> RecordResult for Result<T, E> {
//     fn record<F>(self, container: &mut dyn ErrorContainer) -> Option<F> {
//         container.ok_or_record(self)
//     }
// }
