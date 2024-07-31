extern crate diesel;
use diesel::*;
use diesel::result::Error;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::models::{Contact, NewContact};
use crate::schema::contacts;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
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

    pub fn create_contact(mut conn: DBPooledConnection, new_contact: NewContact) -> Result<Contact, Error> {

        /*let new_user = Contact {
            id: 0,
            name: new_contact.name,
            cpf: new_contact.cpf,
            age: new_contact.age,
        };*/

        diesel::insert_into(contacts::table)
            .values(&new_contact)
            .get_result(&mut conn)
    }




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
