use actix_web::{get, HttpResponse, Responder, web};
# 优化算法效率
use sysinfo::{System, SystemExt};
use std::sync::Arc;

/// 系统性能监控工具
# 优化算法效率
/// 
# TODO: 优化性能
/// 这个程序使用Actix框架来创建一个Web服务，该服务监控系统的CPU和内存使用情况。
/// 

/// 启动web服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建系统信息实例
    let sys = System::new_all(); // 收集所有信息
    let sys = Arc::new(sys); // 使用Arc来共享系统信息实例

    // 启动Actix Web服务器
# 添加错误处理
    actix_web::HttpServer::new(move || {
        let sys = sys.clone(); // 克隆Arc引用
        actix_web::App::new()
            // 定义GET路由，返回系统性能信息
            .service(get_performance_info(sys))
    })
    .bind("127.0.0.1:8080")? // 绑定服务器地址和端口
# FIXME: 处理边界情况
    .run()
    .await
}

/// GET路由处理器，返回系统性能信息
#[get("/performance_info")]
# 优化算法效率
async fn get_performance_info(sys: web::Data<Arc<System>>) -> impl Responder {
    let data = sys.lock().unwrap(); // 锁定系统信息实例
# 优化算法效率

    // 收集CPU和内存使用情况
    let cpu_usage = data.global_processor_info().cpu_usage();
    let memory_usage = data.used_memory() as f32 / data.total_memory() as f32 * 100.0;

    // 创建响应数据
    let response = format!("
# TODO: 优化性能
    {{
        "cpu_usage": {cpu_usage},
        "memory_usage": {memory_usage}
    }}
    ", cpu_usage, memory_usage);

    HttpResponse::Ok().content_type("application/json").body(response)
}