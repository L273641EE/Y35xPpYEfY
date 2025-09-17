use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::path::PathBuf;

/// 文件备份和同步工具的配置信息
struct Config {
    source_path: PathBuf,
    backup_path: PathBuf,
}

/// 文件备份和同步工具
struct FileBackupSync {
    config: Config,
}

impl FileBackupSync {
    /// 创建一个新的文件备份和同步工具实例
    fn new(config: Config) -> Self {
        FileBackupSync { config }
    }

    /// 备份单个文件
    fn backup_file(&self, path: &Path) -> io::Result<()> {
        let source_file = File::open(path)?;
        let backup_file_path = self.config.backup_path.join(path.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "文件名无效"))?);
        let mut backup_file = File::create(&backup_file_path)?;
        io::copy(&mut source_file, &mut backup_file)?;
        Ok(())
    }

    /// 同步目录
    fn sync_directory(&self) -> io::Result<()> {
        let entries = fs::read_dir(&self.config.source_path)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let backup_path = self.config.backup_path.join(path.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "文件名无效"))?);
                fs::create_dir_all(&backup_path)?;
                self.sync_directory(&path)?;
            } else {
                self.backup_file(&path)?;
            }
        }
        Ok(())
    }
}

/// 处理备份单个文件的请求
#[post("/backup_file")]
async fn backup_file_endpoint(file_path: web::Path<String>) -> impl Responder {
    let file_path = Path::new(&file_path);
    if !file_path.exists() {
        return HttpResponse::NotFound().finish();
    }

    let config = Config {
        source_path: PathBuf::from("/source/directory"),
        backup_path: PathBuf::from("/backup/directory"),
    };
    let tool = FileBackupSync::new(config);

    match tool.backup_file(file_path) {
        Ok(_) => HttpResponse::Ok().json("文件备份成功"),
        Err(e) => HttpResponse::InternalServerError().json(format!("备份失败: {}", e)),
    }
}

/// 处理同步目录的请求
#[post("/sync_directory")]
async fn sync_directory_endpoint() -> impl Responder {
    let config = Config {
        source_path: PathBuf::from("/source/directory"),
        backup_path: PathBuf::from("/backup/directory"),
    };
    let tool = FileBackupSync::new(config);

    match tool.sync_directory() {
        Ok(_) => HttpResponse::Ok().json("目录同步成功"),
        Err(e) => HttpResponse::InternalServerError().json(format!("同步失败: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(backup_file_endpoint)
            .service(sync_directory_endpoint)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}