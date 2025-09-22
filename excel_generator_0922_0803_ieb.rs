use actix_web::{get, HttpResponse, Responder, web, App, HttpServer};
use actix_web_validator::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;
use chrono::Local;

// 定义一个结构体来存储生成Excel表格的参数
#[derive(Serialize, Deserialize, Debug)]
pub struct ExcelRequest {
    pub data: Vec<Vec<String>>,
}

// 创建一个服务来生成Excel文件
#[get("/generate_excel")]
async fn generate_excel() -> impl Responder {
    let data = vec![
        vec!["Date".to_string(), "Name".to_string()],
        vec![Local::now().format("%Y-%m-%d").to_string(), "John Doe".to_string()],
    ];

    // 将数据写入Excel文件
    let excel_path = Path::new("output.xlsx");
    match write_excel_file(&excel_path, &data) {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Excel file generated successfully"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// 将数据写入Excel文件的函数
fn write_excel_file(path: &Path, data: &Vec<Vec<String>>) -> Result<(), String> {
    // 这里是一个占位符函数，你需要使用一个Excel库来实现写入功能
    // 例如：`excel_writer::write_file(path, data)?;
    Ok(())
}

// Actix Web服务器配置
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(generate_excel)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 这个程序是一个简单的Excel表格自动生成器，使用RUST和ACTIX框架。
// 它接受一个GET请求来触发Excel文件的生成，并将其保存到本地。
// 请注意，这个示例中没有实现具体的Excel写入功能，你需要使用一个RUST的Excel库
// 例如：`excel_writer`，来完成这个功能。