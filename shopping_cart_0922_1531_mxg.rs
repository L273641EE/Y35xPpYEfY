use actix_web::{
    web,
    HttpResponse,
    HttpServer,
    Responder,
    error,
    Error as ActixError,
};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
# FIXME: 处理边界情况
use uuid::Uuid;

// Define the Product structure
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Product {
    id: Uuid,
    name: String,
    price: f64,
# TODO: 优化性能
}
# FIXME: 处理边界情况

// Define the Cart Item structure
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CartItem {
    product: Product,
    quantity: u32,
}

// Define the Cart structure
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Cart {
    id: Uuid,
    items: Vec<CartItem>,
}

// Define a simple error type for cart operations
#[derive(Debug)]
enum CartError {
    ItemNotFound,
    InvalidQuantity,
}

impl std::fmt::Display for CartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CartError::ItemNotFound => write!(f, "Item not found in the cart"),
            CartError::InvalidQuantity => write!(f, "Invalid quantity"),
        }
    }
# 增强安全性
}

// Implement Responder for CartError to return error responses
impl Responder for CartError {
    type Error = ActixError;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<HttpResponse, ActixError>>>>;
    fn respond_to(self, _: &mut actix_web::dev::ServiceRequest) -> Self::Future {
        Box::pin(async move {
            match self {
                CartError::ItemNotFound => HttpResponse::NotFound().json("Item not found"),
# 增强安全性
                CartError::InvalidQuantity => HttpResponse::BadRequest().json("Invalid quantity"),
# 优化算法效率
            }
        })
    }
}

// Define the handler for adding a product to the cart
async fn add_product_to_cart(item: web::Json<CartItem>) -> impl Responder {
    let cart = web::Data::get::<web::AppData<Cart>>().await;
    let mut items = cart.items.clone();
    let mut found = false;
    // Check if the product is already in the cart
    for cart_item in items.iter_mut() {
        if cart_item.product.id == item.product.id {
            cart_item.quantity += item.quantity;
# NOTE: 重要实现细节
            found = true;
            break;
        }
    }
    if !found {
        items.push(item.clone());
    }
# NOTE: 重要实现细节
    // Update the cart with the new items
    let updated_cart = Cart {
        id: cart.id.clone(),
        items,
    };
    web::Data::set(updated_cart).await;
    HttpResponse::Ok().json(updated_cart)
}

// Define the handler for removing a product from the cart
async fn remove_product_from_cart(product_id: web::Path<Uuid>) -> impl Responder {
    let mut cart = web::Data::get::<web::AppData<Cart>>().await;
    let product_id = product_id.into_inner();
    let mut items = cart.items.clone();
    items.retain(|item| item.product.id != product_id);
    // Update the cart with the new items
# 添加错误处理
    let updated_cart = Cart {
# 扩展功能模块
        id: cart.id.clone(),
        items,
    };
    web::Data::set(updated_cart).await;
    HttpResponse::Ok().json(updated_cart)
}
# 添加错误处理

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the cart with an empty list of items
    let initial_cart = Cart {
# TODO: 优化性能
        id: Uuid::new_v4(),
        items: Vec::new(),
# 改进用户体验
    };
    let cart = web::Data::new(initial_cart);
# 增强安全性
    
    HttpServer::new(move || {
# NOTE: 重要实现细节
        let app = actix_web::App::new()
            .app_data(cart.clone())
            .route("/add", web::post().to(add_product_to_cart))
            .route("/remove/{product_id}", web::delete().to(remove_product_from_cart));
# 增强安全性
        app
    }).
   .bind("127.0.0.1:8080")?
   .run()
   .await
}
# 增强安全性
