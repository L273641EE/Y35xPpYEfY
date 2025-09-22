use actix_web::web;
use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 定义库存项
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    pub id: u32,
    pub name: String,
    pub quantity: u32,
}

// 定义库存管理系统
#[derive(Clone)]
struct InventoryManager {
    items: HashMap<u32, Item>,
}

impl InventoryManager {
    // 新建库存管理系统
    fn new() -> Self {
        InventoryManager {
            items: HashMap::new(),
        }
    }

    // 添加库存项
    fn add_item(&mut self, item: Item) {
        self.items.insert(item.id, item);
    }

    // 获取库存项
    fn get_item(&self, id: u32) -> Option<Item> {
        self.items.get(&id).cloned()
    }

    // 更新库存项数量
    fn update_item_quantity(&mut self, id: u32, quantity: u32) -> Result<(), String> {
        if let Some(item) = self.items.get_mut(&id) {
            item.quantity = quantity;
            Ok(())
        } else {
            Err("Item not found".to_string())
        }
    }
}

// 创建库存管理服务
struct InventoryService {
    manager: InventoryManager,
}

impl InventoryService {
    // 创建服务实例
    fn new() -> Self {
        InventoryService {
            manager: InventoryManager::new(),
        }
    }
}

// 定义库存管理API
#[get("/items/{id}")]
async fn get_item_handler(item_id: web::Path<u32>, service: web::Data<InventoryService>) -> impl Responder {
    match service.manager.get_item(item_id.into_inner()) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().body("Item not found"),
    }
}

#[get("/items")]
async fn get_all_items(service: web::Data<InventoryService>) -> impl Responder {
    HttpResponse::Ok().json(service.manager.items.values().cloned().collect::<Vec<Item>>())
}

#[post("/items")]
async fn add_item_handler(item: web::Json<Item>, service: web::Data<InventoryService>) -> impl Responder {
    service.manager.add_item(item.into_inner().clone());
    HttpResponse::Created().json(item.into_inner())
}

#[put("/items/{id}")]
async fn update_item_handler(item_id: web::Path<u32>, quantity: web::Json<u32>, service: web::Data<InventoryService>) -> impl Responder {
    match service.manager.update_item_quantity(item_id.into_inner(), quantity.into_inner()) {
        Ok(_) => HttpResponse::Ok().json(()),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

// 运行库存管理系统服务
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let inventory_service = web::Data::new(InventoryService::new());
    
    // 注册API路由
    let app = actix_web::App::new()
        .data(inventory_service)
        .service(get_item_handler)
        .service(get_all_items)
        .service(add_item_handler)
        .service(update_item_handler);
    
    // 启动服务
    actix_web::HttpServer::new(|| app.clone())
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
