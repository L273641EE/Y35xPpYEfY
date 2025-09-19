use actix_service::Service;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

// 定义库存项
#[derive(Deserialize, Debug)]
struct InventoryItem {
    id: u32,
    name: String,
    quantity: u32,
}
# TODO: 优化性能

// 库存管理系统服务
struct InventoryService {
    items: web::Data<HashMap<u32, InventoryItem>>,
}

impl InventoryService {
    // 创建新的库存服务实例
    fn new() -> Self {
# 优化算法效率
        InventoryService {
            items: web::Data::new(HashMap::new()),
        }
    }

    // 添加库存项
    async fn add_item(&self, item: InventoryItem) -> impl Responder {
        let mut items = item.id;
        if self.items.contains_key(&item.id) {
            // 如果ID已存在，则返回错误响应
            return HttpResponse::BadRequest().json(json!({"error": "Item already exists"}));
        }

        self.items.insert(item.id, item);
# 添加错误处理
        HttpResponse::Ok().json(json!({"message": "Item added successfully"}))
    }
# 改进用户体验

    // 获取库存项
# TODO: 优化性能
    async fn get_item(&self, id: u32) -> impl Responder {
        match self.items.get(&id) {
            Some(item) => HttpResponse::Ok().json(item),
            None => HttpResponse::NotFound().json(json!({"error": "Item not found"})),
        }
    }

    // 更新库存项
    async fn update_item(&self, id: u32, item: InventoryItem) -> impl Responder {
        match self.items.get_mut(&id) {
            Some(existing_item) => {
                existing_item.name = item.name;
# 添加错误处理
                existing_item.quantity = item.quantity;
                HttpResponse::Ok().json(json!({"message": "Item updated successfully"}))
# 添加错误处理
            },
            None => HttpResponse::NotFound().json(json!({"error": "Item not found"})),
        }
    }

    // 删除库存项
    async fn delete_item(&self, id: u32) -> impl Responder {
        match self.items.remove(&id) {
# 改进用户体验
            Some(_item) => HttpResponse::Ok().json(json!({"message": "Item deleted successfully"})),
            None => HttpResponse::NotFound().json(json!({"error": "Item not found"})),
# 改进用户体验
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(InventoryService::new().items.clone())
# FIXME: 处理边界情况
            .route("/add", web::post().to(InventoryService::add_item))
            .route("/item/{id}", web::get().to(InventoryService::get_item))
            .route("/item/{id}", web::put().to(InventoryService::update_item))
            .route("/item/{id}", web::delete().to(InventoryService::delete_item))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
