// main.rs
use actix_web::web::{Data, Json, Path};

use actix_web::{App, HttpServer, web, HttpResponse, Responder, middleware, get, post};
use diesel::r2d2::ConnectionManager;
use diesel::prelude::*;
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};

mod db;
mod models;
mod constants;
mod schema;
mod services;
mod handlers;
mod ctl_contact;
mod Employee;

use db::establish_connection_pool;
use crate::models::Contact;
use crate::services::service_contacts;
use crate::services::contact_service::ContactService;
use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};

use crate::schema::contacts;
pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type WebPool = Data<Pool<ConnectionManager<PgConnection>>>;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=trace");

    let pool =  establish_connection_pool();

    HttpServer::new(move || {
        App::new() 
            .app_data(  Data::new(pool.clone()) )
            .wrap(middleware::Logger::default())

            .service(get_contacts_v2)
            .service(get_contact_v2)            
            .service(get_contacts)
            .service(ctl_contact::get_contacts_v1)
            .service(ctl_contact::get_contact_v1)
            .service(ctl_contact::create_contact_v1)
            .service(handlers::index)
    })
    .bind("127.0.0.1:8002")?
    .run()
    .await
}


// v1 utiliza ContactService


// v2 utiliza service_contacts
#[get("/v2/contacts")]
async fn get_contacts_v2(db: WebPool) -> HttpResponse {
    let pool: DBPooledConnection = db.get().expect(CONNECTION_POOL_ERROR); 
    let get_users = service_contacts::get_users(pool);
    println!("{:?}", get_users);
    HttpResponse::Ok().content_type(APPLICATION_JSON).json(get_users)
}

#[get("/v2/contacts/{id}")]
async fn get_contact_v2(db: WebPool, path: Path<(i32,)>,) -> impl Responder {
    let pool: DBPooledConnection = db.get().expect(CONNECTION_POOL_ERROR); 
    let user_id = path.0;
    let get_user = service_contacts::get_user(pool, user_id);
    println!("{:?}", get_user);
    HttpResponse::Ok().content_type(APPLICATION_JSON).json(get_user)
}





#[get("/contacts")]
async fn get_contacts(db: WebPool) -> HttpResponse {
    // use crate::contacts::dsl::contacts;
    use crate::schema::contacts::dsl::*;
    let pool: DBPooledConnection = db.get().expect(CONNECTION_POOL_ERROR);

    //let contacts_result: Result<Vec<Contact>, diesel::result::Error> = contacts.load::<Contact>(&pool);

    let likes: Vec<Contact> = match contacts.load::<Contact>(&pool)
    {
        Ok(likes) => likes,
        Err(_) => vec![],
    };
    HttpResponse::Ok().content_type(APPLICATION_JSON).json(likes)

    /*match contacts_result {
        Ok(contacts) => HttpResponse::Ok().content_type("application/json").json(contacts),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }*/
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
