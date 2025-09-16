use actix_web::{get, HttpResponse, Responder, web};
use rand::Rng;
use serde_derive::{Deserialize, Serialize};

/// A simple struct to hold test data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestData {
    pub id: u32,
    pub name: String,
    pub value: f64,
}

/// The TestDataGenerator struct is responsible for generating test data.
pub struct TestDataGenerator;

impl TestDataGenerator {
    /// Generates a new instance of TestData
    pub fn generate_test_data(&self) -> TestData {
        let mut rng = rand::thread_rng();
        let id: u32 = rng.gen();
        let name: String = rng.gen::<u32>().to_string();
        let value: f64 = rng.gen();

        TestData { id, name, value }
    }
}

/// The main application struct which holds the state of the actix web server.
pub struct AppState;

/// Define the route for generating test data
#[get("/test_data")]
async fn generate_test_data_route(generator: web::Data<TestDataGenerator>) -> impl Responder {
    match generator.generate_test_data() {
        test_data => HttpResponse::Ok().json(test_data),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up the logger
    env_logger::init();

    // Define the data generator as shared state
    let data_generator = web::Data::new(TestDataGenerator);

    // Start the actix web server
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(data_generator.clone())
            .route("/test_data", web::get().to(generate_test_data_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
