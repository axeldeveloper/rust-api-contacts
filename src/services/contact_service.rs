extern crate diesel;
use diesel::*;
use diesel::result::Error;
use diesel::r2d2::{ConnectionManager, PooledConnection};
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

use crate::models::Contact;

pub struct ContactService;

impl ContactService {

    pub fn get_contacts(mut conn: DBPooledConnection) -> Result<Vec<Contact>, Error> {  
        use crate::schema::contacts::dsl::*;
        let results = contacts.load::<Contact>(&mut conn);
        return results;
    }

    pub fn get_contact(mut conn: DBPooledConnection, contact_id: i32) -> Result<Contact, Error> {
        use crate::schema::contacts::dsl::*;
        let contact = contacts.filter(id.eq(contact_id)).first(&mut conn);
        return contact;
    }


    
//     pub fn create_contact(
//         conn: &PooledConnection<ConnectionManager<PgConnection>>,
//         new_contact: NewContact,
//     ) -> Result<Contact, Error> {
//         diesel::insert_into(contacts::table)
//             .values(new_contact)
//             .get_result(conn)
//     }


//     pub fn update_contact(
//         conn: &PooledConnection<ConnectionManager<PgConnection>>,
//         contact_id: i32,
//         updated_contact: NewContact,
//     ) -> Result<Contact, Error> {
//         use crate::schema::contacts::dsl::*;
//         diesel::update(contacts.filter(id.eq(contact_id))
//             .set(updated_contact))
//             .get_result(conn)
//     }

//     pub fn delete_contact(
//         conn: &PooledConnection<ConnectionManager<PgConnection>>,
//         contact_id: i32,
//     ) -> Result<(), Error> {
//         use crate::schema::contacts::dsl::*;
//         diesel::delete(contacts.filter(id.eq(contact_id))).execute(conn)?;
//         Ok(())
//     }
}
