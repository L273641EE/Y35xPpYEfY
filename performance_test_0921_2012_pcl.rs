// 引入必要的库
use actix_web::{web, App, HttpServer, Responder, Error};
use std::time::Duration;
# TODO: 优化性能

// 定义一个简单的响应函数，用于性能测试
async fn index() -> Result<impl Responder, Error> {
# FIXME: 处理边界情况
    // 模拟一些工作
    let _ = some_expensive_computation().await;
    // 返回一个简单的响应
    Ok("What's up?")
}

// 模拟一些昂贵的计算
async fn some_expensive_computation() -> () {
    // 这里只是一个占位符，实际应用中可以替换为实际的计算
# FIXME: 处理边界情况
    tokio::time::sleep(Duration::from_secs(1)).await;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
# NOTE: 重要实现细节
        App::new()
            // 添加我们的端点
            .route("/", web::get().to(index))
    })
    // 绑定到端口8080
    .bind("127.0.0.1:8080")?
    // 开始监听请求
    .run()
    // 将错误传递给上层处理
    .await
}
