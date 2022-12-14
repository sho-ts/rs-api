use serde::Serialize;

#[derive(Serialize)]
pub struct HelloUseCaseOutPut {
    pub message: String
}