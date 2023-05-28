#[derive(Debug)]
pub struct ErrorMessage {
    pub message: String,
}

impl ErrorMessage {
    pub fn new<M>(message: M) -> Self
    where
        M: std::fmt::Display,
    {
        let message = message.to_string();
        Self { message }
    }
}

impl warp::reject::Reject for ErrorMessage {}
