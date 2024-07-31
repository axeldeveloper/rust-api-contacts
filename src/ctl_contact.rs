use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use actix_web::web::{Json, Path};
use crate::{DBPooledConnection, WebPool};
use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::models::{Contact, NewContact};
use crate::services::contact_service::ContactService;

#[get("/v1/contacts")]
async fn get_contacts_v1(db: WebPool) -> HttpResponse {
    let pool: DBPooledConnection = db.get().expect(CONNECTION_POOL_ERROR);
    let get_users = ContactService::get_contacts(pool);
    println!("{:?}", get_users);
    match get_users {
        Ok(get_users) => HttpResponse::Ok().content_type(APPLICATION_JSON).json(get_users),
        Err(_) => HttpResponse::NoContent().await.unwrap(),
    }
}

#[get("/v1/contacts/{id}")]
async fn get_contact_v1(param: Path<u32>, db: WebPool) -> HttpResponse {
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

#[post("/v1/contacts")]
async fn create_contact_v1(body: Json<NewContact>, db: WebPool) -> HttpResponse {
    let pool: DBPooledConnection = db.get().expect(CONNECTION_POOL_ERROR);

    // let new_contact = NewContact {
    //     name: body.name.to_string(),
    //     cpf: body.cpf.to_string(),
    //     age: body.age,
    // };

    let new_contact =  body.into_inner();

    let event =  ContactService::create_contact(pool, new_contact);
    match event {
        Ok(event)=>HttpResponse::Ok().json(event),
        Err(_)=>HttpResponse::InternalServerError().body("Internal Server Error")
    }

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
}