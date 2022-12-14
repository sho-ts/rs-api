use crate::application::{
    input::hello::hello_usecase_input::HelloUseCaseInput,
    output::hello::hello_usecase_output::HelloUseCaseOutPut,
};

pub fn handle(input: HelloUseCaseInput) -> HelloUseCaseOutPut {
    HelloUseCaseOutPut {
        message: input.message,
    }
}
