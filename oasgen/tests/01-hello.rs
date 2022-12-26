use oasgen::{OaSchema, Server, openapi};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, OaSchema)]
pub struct SendCode {
    pub mobile: String,
}

#[derive(Serialize, OaSchema)]
pub struct SendCodeResponse {
    pub found_account: bool,
}

#[openapi]
async fn send_code(_body: Json<SendCode>) -> Json<SendCodeResponse> {
    Json(SendCodeResponse { found_account: false })
}

#[test]
fn test_basic_actix() {
    use std::fs::File;
    let s = Server::new()
        // .get("/hello", send_code)
        ;
    serde_yaml::to_writer(&File::create("tests/01-hello.yaml").unwrap(), &s.openapi).unwrap();
    println!("{:?}", s.openapi);
    assert_eq!(1, 0);
}