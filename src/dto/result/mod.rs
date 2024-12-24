pub struct ResultDTO<T> {
    success: bool,
    result: Option<T>,
    reason: Option<String>,
}

impl<T> ResultDTO<T> {
    fn new(success: bool, result: Option<T>, reason: &str) -> Self {
        ResultDTO {
            success,
            result,
            reason: if !reason.is_empty() {
                Some(reason.to_owned())
            } else {
                None
            },
        }
    }

    pub fn create_success_result(result: Option<T>) -> Self {
        return Self::new(true, result, "");
    }

    pub fn create_failed_result(reason: &str) -> Self {
        return Self::new(false, None, reason);
    }

    pub fn is_success(&self) -> bool {
        self.success
    }

    pub fn get_result(self) -> Option<T> {
        self.result
    }

    pub fn get_reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}
