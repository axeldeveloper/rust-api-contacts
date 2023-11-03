// main.rs
use actix_web::web::{Data, Path};
use actix_web::{App, HttpServer, web, HttpResponse, Responder, middleware, get};
use diesel::r2d2::ConnectionManager;
use diesel::prelude::*;
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};

mod db;
mod models;
mod constants;
mod schema;
mod services;

use db::establish_connection_pool;
use crate::models::Contact;
use crate::schema::contacts;
use crate::services::service_contacts;
use crate::services::contact_service::ContactService;
use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type WebPool = web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=trace");
    
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    // let manager = ConnectionManager::<PgConnection>::new(database_url);
    // let pool = r2d2::Pool::builder()
    //     .build(manager)
    //     .expect("Failed to create connection pool");
    
    let pool =  establish_connection_pool();

    HttpServer::new(move || {
        App::new() 
            .app_data(  Data::new(pool.clone()) )
            .wrap(middleware::Logger::default())
            .service(get_contacts_v1)
            .service(get_contacts_v2)
            .service(get_contact_v2)            
            .service(get_contacts)
            .service(get_contact_v1)     
    })
    .bind("127.0.0.1:8002")?
    .run()
    .await
}


#[get("/v1/contacts")]
async fn get_contacts_v1(db: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    let pool: DBPooledConnection = db.get().expect(CONNECTION_POOL_ERROR); 
    let get_users = service_contacts::get_users(pool);
    println!("{:?}", get_users);
    HttpResponse::Ok().content_type(APPLICATION_JSON).json(get_users)
}

#[get("/v1/contacts/{id}")]
async fn get_contact_v1(db: WebPool, path: Path<(i32,)>,) -> impl Responder {
    let pool: DBPooledConnection = db.get().expect(CONNECTION_POOL_ERROR); 
    let user_id = path.0;
    let get_user = service_contacts::get_user(pool, user_id);
    println!("{:?}", get_user);
    HttpResponse::Ok().content_type(APPLICATION_JSON).json(get_user)
}


#[get("/v2/contacts")]
async fn get_contacts_v2(db: WebPool) -> HttpResponse {
    let pool: DBPooledConnection = db.get().expect(CONNECTION_POOL_ERROR); 
    //let get_users = service_contacts::get_users(pool);
    let get_users = ContactService::get_contacts(pool);
    println!("{:?}", get_users);
    match get_users {
        Ok(get_users) => HttpResponse::Ok().content_type(APPLICATION_JSON).json(get_users),
        Err(_) => HttpResponse::NoContent().await.unwrap(),
    }
}

#[get("/v2/contacts/{id}")]
async fn get_contact_v2(param: Path<u32>, db: WebPool) -> HttpResponse {
    let pool: DBPooledConnection = db.get().expect(CONNECTION_POOL_ERROR);
    let user_id = param.into_inner();
    println!("Path param {}", user_id);
    let contact_id :i32 = user_id.try_into().unwrap();
    let get_user = ContactService::get_contact(pool, contact_id);
    println!("{:?}", get_user);
    match get_user {
        Ok(get_user) => HttpResponse::Ok().content_type(APPLICATION_JSON).json(get_user),
        Err(_) => HttpResponse::NoContent().await.unwrap(),
    }
}



#[get("/contacts")]
async fn get_contacts(db: WebPool) -> HttpResponse {
    use crate::contacts::dsl::contacts;
    let mut pool: DBPooledConnection = db.get().expect(CONNECTION_POOL_ERROR);
    let likes: Vec<Contact> = match contacts.load::<Contact>(&mut pool)
    {
        Ok(lks) => lks,
        Err(_) => vec![],
    };
    HttpResponse::Ok().content_type(APPLICATION_JSON).json(likes)
}


// async fn create_contact(
//     db: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
//     contact: web::Json<models::NewContact>,
// ) -> impl Responder {
//     let pool = db.get().expect("Failed to get database connection from pool");
//     let result = web::block(move || {
//         diesel::insert_into(contacts::table)
//             .values(contact.into_inner())
//             .get_result::<Contact>(&pool.get())
//     })
//     .await
//     .map_err(|e| {
//         eprintln!("{}", e);
//         HttpResponse::InternalServerError()
//     });

//     match result {
//         Ok(data) => HttpResponse::Ok().json(data),
//         Err(resp) => resp,
//     }
// }



// async fn update_contact(
//     db: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
//     path: web::Path<(i32,)>,
//     contact: web::Json<models::NewContact>,
// ) -> impl Responder {
//     let pool = db.get().expect("Failed to get database connection from pool");
//     let contact_id = path.0;
//     let result = web::block(move || {
//         use crate::schema::contacts::dsl::*;
//         let updated_contact = diesel::update(contacts.filter(id.eq(contact_id)))
//             .set(contact.into_inner())
//             .get_result::<Contact>(&pool.get());
//         Ok(updated_contact)
//     })
//     .await
//     .map_err(|e| {
//         eprintln!("{}", e);
//         HttpResponse::InternalServerError()
//     });

//     match result {
//         Ok(data) => HttpResponse::Ok().json(data),
//         Err(resp) => resp,
//     }
// }

// async fn delete_contact(
//     db: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
//     path: web::Path<(i32,)>,
// ) -> impl Responder {
//     let pool = db.get().expect("Failed to get database connection from pool");
//     let contact_id = path.0;
//     let result = web::block(move || {
//         use crate::schema::contacts::dsl::*;
//         diesel::delete(contacts.filter(id.eq(contact_id))).execute(&pool.get())?;
//         Ok(())
//     })
//     .await
//     .map_err(|e| {
//         eprintln!("{}", e);
//         HttpResponse::InternalServerError()
//     });

//     match result {
//         Ok(_) => HttpResponse::Ok(),
//         Err(resp) => resp,
//     }
// }
