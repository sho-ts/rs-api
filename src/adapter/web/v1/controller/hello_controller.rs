use crate::adapter::web::v1::request::hello_controller_request::HelloRequest;
use crate::application::{
    input::hello::hello_usecase_input::HelloUseCaseInput,
    output::hello::hello_usecase_output::HelloUseCaseOutPut, usecase::hello::hello_usecase::handle,
};
use axum::response::Json;

pub async fn hello(Json(payload): Json<HelloRequest>) -> Json<HelloUseCaseOutPut> {
    let input = HelloUseCaseInput {
        message: payload.message,
    };
    let output = handle(input);

    return Json(output);
}
