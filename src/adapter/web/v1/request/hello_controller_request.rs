use serde::Deserialize;

#[derive(Deserialize)]
pub struct HelloRequest {
    pub message: String,
}