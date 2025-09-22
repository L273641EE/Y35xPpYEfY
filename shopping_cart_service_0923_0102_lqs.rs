// shopping_cart_service.rs
// 实现购物车功能的服务
# NOTE: 重要实现细节

use actix_web::{
    web,
    Error,
    get,
    post,
    put,
    delete,
    HttpServer,
    Responde,
    HttpRequest,
    HttpResponse
};

// 购物车中的物品
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CartItem {
    id: i32,
    name: String,
    quantity: i32,
    price: f32,
}

// 购物车服务
# 改进用户体验
struct ShoppingCartService {
# NOTE: 重要实现细节
    cart: Vec<CartItem>,
}

impl ShoppingCartService {
    // 创建新购物车服务
    fn new() -> ShoppingCartService {
        ShoppingCartService {
            cart: Vec::new(),
        }
    }

    // 添加商品到购物车
    fn add_item(&mut self, item: CartItem) -> Result<(), Error> {
        self.cart.push(item);
        Ok(())
# 添加错误处理
    }

    // 从购物车中移除商品
# 改进用户体验
    fn remove_item(&mut self, item_id: i32) -> Result<(), Error> {
        self.cart.retain(|item| item.id != item_id);
        Ok(())
    }
# 增强安全性

    // 更新购物车中的商品数量
    fn update_item_quantity(&mut self, item_id: i32, new_quantity: i32) -> Result<(), Error> {
        if let Some(item) = self.cart.iter_mut().find(|item| item.id == item_id) {
            item.quantity = new_quantity;
            Ok(())
        } else {
            Err(actix_web::error::ErrorNotFound("Item not found"))
# 优化算法效率
        }
    }

    // 获取购物车中所有商品
    fn get_cart(&self) -> Result<Vec<CartItem>, Error> {
        Ok(self.cart.clone())
    }
# 增强安全性
}

// 定义购物车服务的HTTP接口
async fn add_to_cart(item: web::Json<CartItem>) -> impl Responde {
    let mut service = ShoppingCartService::new();
    match service.add_item(item.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Item added to cart"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to add item to cart"),
# 添加错误处理
    }
}

async fn remove_from_cart(item_id: web::Path<i32>) -> impl Responde {
    let mut service = ShoppingCartService::new();
    match service.remove_item(item_id.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Item removed from cart"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to remove item from cart"),
    }
}

async fn update_cart_quantity(item_id: web::Path<i32>, quantity: web::Json<i32>) -> impl Responde {
    let mut service = ShoppingCartService::new();
    match service.update_item_quantity(item_id.into_inner(), quantity.into_inner()) {
        Ok(_) => HttpResponse::Ok().json("Item quantity updated"),
# 扩展功能模块
        Err(_) => HttpResponse::InternalServerError().json("Failed to update item quantity"),
    }
# TODO: 优化性能
}
# 改进用户体验

async fn get_cart_items() -> impl Responde {
    let service = ShoppingCartService::new();
# NOTE: 重要实现细节
    match service.get_cart() {
        Ok(items) => HttpResponse::Ok().json(items),
# 增强安全性
        Err(_) => HttpResponse::InternalServerError().json("Failed to get cart items"),
# 扩展功能模块
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
# 优化算法效率
    HttpServer::new(|| {
        let service = ShoppingCartService::new();
        actix_web::App::new()
            .app_data(web::Data::new(service))
            .route("/add", post().to(add_to_cart))
            .route("/remove/{item_id}", delete().to(remove_from_cart))
            .route("/update/{item_id}", put().to(update_cart_quantity))
            .route("/cart", get().to(get_cart_items))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
