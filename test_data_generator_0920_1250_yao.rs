use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use rand::distributions::{Alphanumeric, Distribution, Uniform};
use rand::{thread_rng, Rng};
use std::collections::HashMap;

// 测试数据生成器结构体
struct TestDataGenerator;

// 实现Actix Web的Handler特性，处理HTTP请求
impl actix_web::dev::HttpServiceFactory for TestDataGenerator {
    type Config = actix_web::web::ServiceConfig;
    type Response = actix_web::dev::Handler;
    type Error = actix_web::Error;
    type InitError = actix_web::dev::InitError;

    fn new() -> Self {
        TestDataGenerator
    }

    fn register(self, config: &mut Self::Config) {
        config.service(
            self.resource().with(get().to(|| async {
                HttpResponse::Ok().json(generate_test_data())
            }))
        );
    }
}

// 生成测试数据
fn generate_test_data() -> HashMap<String, String> {
    let mut rng = thread_rng();
    let mut data = HashMap::new();

    data.insert(
        "username".to_string(),
        rng.sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>()
    );

    data.insert(
        "password".to_string(),
        rng.sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect::<String>()
    );

    data.insert(
        "email".to_string(),
        format!("{}@example.com", rng.sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>())
    );

    data.insert(
        "age".to_string(),
        rng.gen_range(18..100).to_string()
    );

    data
}

// 启动Actix Web服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(TestDataGenerator::new()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
