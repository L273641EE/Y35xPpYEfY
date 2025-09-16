use actix_web::{web, App, HttpServer, Responder, HttpResponse, error::{ErrorInternalServerError, ErrorBadRequest}};
use zip::ZipArchive;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::fs::{self, File};
use std::env;

/// 解压文件到指定目录
async fn unzip_handler(path: web::Path<String>) -> Result<impl Responder, ErrorBadRequest> {
    let source = path.into_inner();
    let dest = env::temp_dir().join("unzipped");
    
    // 检查源文件是否存在
    if !PathBuf::from(&source).exists() {
        return Err(ErrorBadRequest("Source file does not exist"));
    }
    
    // 读取ZIP文件并解压
    let mut file = File::open(&source).map_err(|_| ErrorBadRequest("Failed to open source file"))?;
    let mut archive = ZipArchive::new(file).map_err(|_| ErrorBadRequest("Failed to read ZIP file"))?;
    
    if !dest.exists() {
        fs::create_dir_all(&dest).map_err(|_| ErrorBadRequest("Failed to create destination directory"))?;
    }
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|_| ErrorBadRequest("Failed to read file in ZIP"))?;
        let outpath = dest.join(file.sanitized_name());
        
        if file.name().ends_with('/') { // 目录则创建
            fs::create_dir_all(&outpath).map_err(|_| ErrorBadRequest("Failed to create directory"))?;
        } else { // 文件则解压
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).map_err(|_| ErrorBadRequest("Failed to create parent directory"))?;
                }
            }
            let mut outfile = File::create(&outpath).map_err(|_| ErrorBadRequest("Failed to create output file"))?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).map_err(|_| ErrorBadRequest("Failed to read file content"))?;
            outfile.write_all(&contents).map_err(|_| ErrorBadRequest("Failed to write file content"))?;
        }
    }
    
    Ok(HttpResponse::Ok().json({