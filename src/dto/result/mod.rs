pub struct ResultDTO<T> {
    result: Option<T>,
    reason: String,
}

impl<T> ResultDTO<T> {
    fn new(result: Option<T>, reason: &str) -> Self {
        ResultDTO {
            result,
            reason: reason.to_owned(),
        }
    }

    pub fn create_success_result(result: Option<T>) -> Self {
        return Self::new(result, "");
    }

    pub fn create_failed_result(reason: &str) -> Self {
        return Self::new(None, reason);
    }

    pub fn get_result(&self) -> &Option<T> {
        &self.result
    }

    pub fn get_reason(&self) -> &str {
        self.reason.as_str()
    }
}
