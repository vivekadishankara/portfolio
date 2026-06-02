use leptos::prelude::ServerFnError;

/// Extension trait to convert any `Display` error into a `ServerFnError`.
pub trait IntoServerError<T> {
    fn server_err(self) -> Result<T, ServerFnError>;
}

impl<T, E: std::fmt::Display> IntoServerError<T> for Result<T, E> {
    fn server_err(self) -> Result<T, ServerFnError> {
        self.map_err(|e| ServerFnError::new(e.to_string()))
    }
}
