use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// 定义一个全局变量来存储当前的主题
static THEME: std::sync::Mutex<String> = std::sync::Mutex::new(String::from("light"));

// 定义一个主题枚举
#[derive(Debug, Clone)]
enum Theme {
    Light,
    Dark,
}

// 将主题枚举转换为字符串
impl From<Theme> for String {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => "light".into(),
            Theme::Dark => "dark".into(),
        }
    }
}

// 主题切换功能
#[post("/switch_theme")]
async fn switch_theme() -> impl Responder {
    // 获取当前主题并切换
    let mut theme = THEME.lock().unwrap();
    if theme == "light" {
        *theme = Theme::Dark.into();
    } else {
        *theme = Theme::Light.into();
    }
    // 返回成功响应
    HttpResponse::Ok().body("Theme switched successfully!")
}

// 获取当前主题
#[get("/get_theme")]
async fn get_theme() -> impl Responder {
    // 获取当前主题
    let theme = THEME.lock().unwrap().clone();
    // 返回当前主题
    HttpResponse::Ok().json(theme)
}

// 主函数，启动服务器
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化服务器
    HttpServer::new(|| {
        App::new()
            // 注册主题切换和获取主题的路由
            .service(switch_theme)
            .service(get_theme)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
