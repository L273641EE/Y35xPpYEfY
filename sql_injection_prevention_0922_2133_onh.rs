use actix_web::{get, HttpResponse, Responder, post, web, App, HttpServer, Responder as ActixResponder, Error as ActixError};
    use diesel::prelude::*;
    use diesel::pg::PgConnection;
    use diesel::r2d2::{self, ConnectionManager};

    /// 连接池配置
    type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

    /// 数据库连接池
    #[derive(Clone)]
    pub struct AppState {
        pub pool: Pool,
    }

    /// 创建数据库连接池
    async fn init_db_pool() -> Pool {
        let manager = ConnectionManager::<PgConnection>::new(
            "postgres://username:password@localhost/database_name");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        pool
    }

    /// 获取用户信息
    #[get("/users/{id}")] // 路由装饰器
    async fn get_user(info: web::Path<i32>, app_state: web::Data<AppState>) -> ActixResponder {
        let conn = app_state.pool.get().map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))?;
        let user_id = info.into_inner();

        use schema::users::dsl::*;

        let result = users.filter(id.eq(user_id)).first(&conn).optional();

        match result {
            Ok(Some(user)) => HttpResponse::Ok().json(user),
            Ok(None) => HttpResponse::NotFound().body(