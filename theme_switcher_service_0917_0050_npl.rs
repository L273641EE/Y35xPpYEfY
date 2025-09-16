use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, Error, Result};
use std::sync::Mutex;
use lazy_static::lazy_static;

// 定义主题枚举
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Theme {
    Light,
    Dark,
}

// 全局状态管理主题
lazy_static! {
    static ref THEME: Mutex<Theme> = Mutex::new(Theme::Light);
}

// 当前主题的上下文提取器
struct ThemeContext(Theme);

impl<'a> actix_web::FromRequest<'a> for ThemeContext {
    type Error = actix_web::Error;
    type Future = actix_web::web::Ready<Result<Self, Self::Error>>;
    type Config = ();

    // 从全局状态中提取当前主题
    fn from_request(req: &'a actix_web::dev::ServiceRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let theme = THEME.lock().unwrap();
        actix_web::web::ready(Ok(ThemeContext(*theme)))
    }
}

// 获取当前主题的路由处理函数
#[get(