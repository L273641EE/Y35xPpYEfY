use actix_web::{web, App, HttpResponse, HttpServer, Responder};

/// 定义一个简单的HTTP请求处理器
/// 这个处理器将返回一个简单的JSON响应
async fn index() -> impl Responder {
    // 构造响应内容
    let content = r#"{\"message\": \"Welcome to the actix-web app!\"}