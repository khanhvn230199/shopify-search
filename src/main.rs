mod db;
mod repositories;
mod models;
mod services; 
mod router;
use dotenv::dotenv;
use db::pool::PostgresPool;
use actix_web::{
    web::Data,
    App, Error, HttpServer, middleware::Logger, body::MessageBody, dev::{ServiceFactory, ServiceRequest, ServiceResponse}
};
use shopify_search::{
    services::user_service::{interface::UserService, user_serviece::UserServiceImpl},
    repositories:: user_repository::interface::UserRepository,
    repositories::postgres_repository::PostgresRepository,
    router::auth_router::init_auth_routes,
    utils::hugging_face::embedding::get_embedding_async,
};
use std::sync::Arc;
pub struct Container {
    pub user_service: Arc<dyn UserService>,
}

impl Container {
    // Thay đổi new() thành async
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Khởi tạo pool
        let postgres_pool = PostgresPool::new().await?;

        // Tạo repository với pool
        let user_repository: Arc<dyn UserRepository + Send + Sync> = Arc::new(
            PostgresRepository::new(postgres_pool.get_pool().clone()) // Truyền pool vào repository
        );

        // Tạo user service
        let user_service: Arc<UserServiceImpl> = Arc::new(
            UserServiceImpl { user_repository: user_repository.clone() }
        );

        Ok(Container {
            user_service,
        })
    }
}

pub fn create_app(container: Arc<Container>) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let user_service: Arc<dyn UserService> = container.user_service.clone();

    App::new()
        .app_data(Data::from(user_service))
        .wrap(Logger::default())
        // .wrap(ServiceContextMaintenanceCheck)
        .configure(init_auth_routes)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
     // Gọi trước khi chạy server hoặc song song tùy bạn
     if let Err(err) = get_embedding_async().await {
        eprintln!("Error in embedding: {}", err);
    }

    let address = "127.0.0.1:8080";
    let container: Arc<Container> = Arc::new(Container::new().await.unwrap());
    let server = HttpServer::new(move || {
        create_app(container.clone()) // Đảm bảo container được clone
    })
    .bind(&address)?;
    server.run().await
}