/// Service functions around creating and getting users
/// I made a point to have every service return `Option<T>`
/// Just for consistently.

extern crate diesel;
use diesel::*;
use crate::models::Contact;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub fn get_users(mut conn: PooledConnection<ConnectionManager<PgConnection>>) -> Option<Vec<Contact>> {
    use crate::schema::contacts::dsl::*;
    let likes = match contacts.load::<Contact>(&mut conn) {
        Ok(likes) => Some(likes),
        Err(_) => None,
    };
    return likes;
}

pub fn get_user(mut conn: PooledConnection<ConnectionManager<PgConnection>>,  user_id: i32) -> Option<Contact> {
    use crate::schema::contacts::dsl::*; 
    //let results = contacts.filter(id.eq(user_id)).limit(1).load::<Contact>(&conn).expect("Error finding user");
    let results = contacts.filter(id.eq(user_id)).first(&mut conn);       
    let row = match results {
        Ok(row) => Some(row),
        Err(_) => None,
    };
    return row;
}



// pub fn get_user(
//     conn: &PooledConnection<ConnectionManager<PgConnection>>,
//     user_id: i32) -> Option<Contact> {

//     let results = contacts.filter(id.eq(user_id)).limit(1).load::<Contact>(&conn).expect("Error finding user");
//     if results.len() == 0 {
//         return None
//     }
//     return Some(results[0].clone())
// }


// pub fn create_user(user: UserRegister) -> Result<User, String> {
//     use schema::users;
//     let connection = establish_connection();
//     let user_salt = password::generate_salt();
//     let user_hash = password::hash_password(&user.password, &user_salt).unwrap();
//     let new_user = NewUser {
//         name: user.name,
//         email: user.email,
//         password_hash: user_hash,
//         salt: user_salt
//     };

//     match diesel::insert(&new_user)
//         .into(users::table)
//         .get_result(&connection) {
//         Ok(t) => return Ok(t),
//         Err(e) => return Err(e.to_string())
//     }
//}
