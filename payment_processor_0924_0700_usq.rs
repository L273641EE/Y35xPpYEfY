use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

/// 支付请求的结构体
#[derive(Serialize, Deserialize)]
struct PaymentRequest {
    /// 支付金额，单位为元
    amount: f64,
    /// 支付方式
    payment_method: String,
}

/// 支付响应的结构体
#[derive(Serialize, Deserialize)]
struct PaymentResponse {
    /// 支付结果
    status: String,
    /// 支付金额
    amount: f64,
}

/// 支付处理器状态
struct PaymentProcessor {
    /// 交易记录
    transactions: Mutex<HashMap<String, PaymentResponse>>,
}

impl PaymentProcessor {
    /// 新建支付处理器
    fn new() -> Self {
        PaymentProcessor {
            transactions: Mutex::new(HashMap::new()),
        }
    }

    /// 处理支付请求
    async fn process_payment(&self, req: PaymentRequest) -> impl Responder {
        let mut transactions = self.transactions.lock().unwrap();

        // 检查支付请求的有效性
        if req.amount <= 0.0 {
            return HttpResponse::BadRequest().json(PaymentResponse {
                status: 