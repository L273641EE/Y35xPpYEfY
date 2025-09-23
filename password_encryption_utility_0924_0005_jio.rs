use actix_web::{get, HttpResponse, web, App, HttpServer, Responder};
use openssl::symm::{encrypt_aead, decrypt_aead};
use openssl::symm::{Cipher, Crypter, Mode, StreamCipher};
use openssl::error::ErrorStack;
use rand::{distributions::Alphanumeric, Rng};
use base64::{encode, decode};
use std::str;

/// 加密密码
#[get("/encrypt/{password}")]
async fn encrypt_password(password: web::Path<String>) -> impl Responder {
    let encrypted = encrypt(&password).unwrap_or_else(|_| "".to_string());
    HttpResponse::Ok().json(encrypted)
}

/// 解密密码
#[get("/decrypt/{encrypted}")]
async fn decrypt_password(encrypted: web::Path<String>) -> impl Responder {
    let decrypted = decrypt(&encrypted).unwrap_or_else(|_| "".to_string());
    HttpResponse::Ok().json(decrypted)
}

/// 加密函数
fn encrypt(input: &str) -> Result<String, ErrorStack> {
    let cipher = Cipher::aes_256_gcm();
    let key = generate_key();
    let iv = generate_iv();
    let input_bytes = input.as_bytes();
    let mut out = vec![0; input_bytes.len() + cipher.block_size() as usize];
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, Some(key.as_ref()), Some(iv.as_ref())).map_err(|e| e)?;
    crypter.update(input_bytes, &mut out).map_err(|e| e)?;
    crypter.finalize(&mut out).map_err(|e| e)?;
    Ok(encode(&out))
}

/// 解密函数
fn decrypt(input: &str) -> Result<String, ErrorStack> {
    let input_bytes = decode(input).map_err(|e| e)?;
    let cipher = Cipher::aes_256_gcm();
    let key = generate_key();
    let iv = generate_iv();
    let mut out = vec![0; input_bytes.len() - cipher.block_size() as usize];
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, Some(key.as_ref()), Some(iv.as_ref())).map_err(|e| e)?;
    crypter.update(&input_bytes, &mut out).map_err(|e| e)?;
    crypter.finalize(&mut out).map_err(|e| e)?;
    Ok(str::from_utf8(&out).map_err(|e| e)?.to_string())
}

/// 生成密钥
fn generate_key() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let key: String = rng.sample_iter(&Alphanumeric).take(32).map(char::from).collect();
    key.as_bytes().to_vec()
}

/// 生成IV
fn generate_iv() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let iv: String = rng.sample_iter(&Alphanumeric).take(12).map(char::from).collect();
    iv.as_bytes().to_vec()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(encrypt_password)
            .service(decrypt_password)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}