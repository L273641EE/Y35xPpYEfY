use actix_web::{web, App, HttpServer, Responder};
use rand::{distributions::{Alphanumeric, Distribution, Standard}, Rng};
use serde::Serialize;
# 添加错误处理
use serde_json::json;

#[derive(Serialize)]
struct TestData {
# 扩展功能模块
    pub id: u64,
    pub name: String,
# 改进用户体验
    pub email: String,
}

/// 生成测试数据的函数
///
/// 该函数生成包含id、name和email的测试数据
async fn generate_test_data() -> impl Responder {
    // 随机生成ID
    let id: u64 = rand::thread_rng().gen();
    
    // 随机生成Name，长度为5-10个字符
    let name: String = rand::thread_rng().sample_iter(&Alphanumeric)
        .take(5..=10)
        .map(char::from)
# 改进用户体验
        .collect();
    
    // 随机生成Email，格式为name@domain.com
# 优化算法效率
    let email: String = format!("{}@domain.com", name);
    
    // 创建TestData实例
# NOTE: 重要实现细节
    let test_data = TestData { id, name, email };
    
    // 将TestData实例序列化为JSON
    json!(test_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            .route("/test-data", web::get().to(generate_test_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
# 添加错误处理
    .await
}
