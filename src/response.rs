use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    status: u16,
    message: String,
    data: T,
}

// Implementasi response wrapper
impl<T: Serialize> ApiResponse<T> {
    fn new(status: u16, message: String, data: T) -> Self {
        Self {
            status,
            message,
            data,
        }
    }

    pub fn ok(data: T) -> Self {
        Self::new(200, "OK".to_string(), data)
    }
}
