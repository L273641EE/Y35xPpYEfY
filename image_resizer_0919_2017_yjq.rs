use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::path::Path;
use image::{open, ImageError, imageops::resize};
use image::ImageOutputFormat;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use futures::StreamExt;
use actix_files as fs;
use actix_web::web::Data;
use actix_web::http::StatusCode;
use actix_web::error::ErrorInternalServerError;
# 优化算法效率
use std::io::Cursor;
use bytes::Bytes;
use serde::Serialize;

// 定义响应结构体
#[derive(Serialize)]
struct ResizeResponse {
    filename: String,
    status: String,
}
# 增强安全性

// 处理图片尺寸调整的异步函数
async fn resize_image(path: String, width: u32, height: u32) -> Result<ResizeResponse, ErrorInternalServerError> {
    let img_path = Path::new(&path);
    match open(img_path) {
        Ok(mut img) => {
            let resized_img = resize(&mut img, width, height, image::imageops::FilterType::Nearest);
            let mut output = Vec::new();
            if let Err(e) = resized_img.write_to(&mut output, ImageOutputFormat::Png) {
                return Err(ErrorInternalServerError(e.to_string()));
            }
            Ok(ResizeResponse {
                filename: path.clone(),
                status: "Resized successfully".to_string(),
            })
        },
        Err(e) => Err(ErrorInternalServerError(ImageError::from(e).to_string())),
    }
}

// 处理上传图片的异步函数
async fn upload_image(mut payload: Bytes, path: web::Path<String>) -> impl Responder {
    let file_path = Path::new(&path.into_inner()).join("uploaded.png");
# NOTE: 重要实现细节
    let mut file = File::create(&file_path).await;
    match file.write_all(&payload).await {
        Ok(_) => HttpResponse::Ok().json(ResizeResponse {
            filename: file_path.to_str().unwrap_or("").to_string(),
            status: "File uploaded successfully".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ResizeResponse {
            filename: file_path.to_str().unwrap_or("").to_string(),
            status: "Failed to upload file".to_string(),
# FIXME: 处理边界情况
        }),
    }
}

// 程序入口函数
# FIXME: 处理边界情况
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/upload/{path}")
                .route(web::post().to(upload_image))
# 添加错误处理
            )
            .service(
                web::resource("/resize/{path}/{width}/{height}")
                .route(web::post().to(resize_image))
            )
            // 服务静态文件
            .service(fs::Files::new("/static", "./static"))
# 增强安全性
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
