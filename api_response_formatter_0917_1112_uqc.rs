// api_response_formatter.rs
// 这是一个使用Actix框架创建的API响应格式化工具。
use actix_web::{
    get,
    web,
    App,
    HttpServer,
    HttpResponse,
    Responder,
};
use serde_json::json;
use uuid::Uuid;

// 定义一个结构体来表示API响应的通用格式。
struct ApiResponse<T> {
    id: Uuid,
    status: String,
    data: T,
    error: Option<String>,
}

// 实现Responder特性，使ApiResponse可以被直接返回作为响应。
impl<T: serde::Serialize> Responder for ApiResponse<T> {
    type Error = actix_web::Error;
    type Future = std::pin::Pin<core::future::ready<actix_web::Result<HttpResponse, Self::Error>>>;
    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        Box::pin(async move {
            Ok(HttpResponse::Ok().json(self))
        })
    }
}

// 实现一个Handler，返回一个格式化的API响应。
#[get("/format")]
async fn format_response() -> impl Responder {
    let response_data = json!({
        "message": "Hello, World!"
    });

    ApiResponse {
        id: Uuid::new_v4(),
        status: "success".to_string(),
        data: response_data,
        error: None,
    }
}

// 启动服务器的主函数。
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(format_response)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
