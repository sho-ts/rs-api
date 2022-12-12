use axum::response::Json;
use serde::{Serialize,};

#[derive(Serialize)]
pub struct Res {
    message: String,
}

pub async fn hello() -> Json<Res> {
    let foo = Res {
        message: String::from("hello"),
    };

    return Json(foo);
}
