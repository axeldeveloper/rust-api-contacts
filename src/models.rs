//use diesel::{sql_types::Bool};
use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

use crate::schema::contacts; // Importe o módulo schema para acessar a tabela contacts


#[derive(Debug, Insertable, Deserialize, Serialize, Queryable)]
#[table_name = "contacts"]
pub struct NewContact {
    pub name: String,
    pub cpf: String,
    pub age: Option<i32>,
   //pub published: bool,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Contact {
    pub id: i32,
    pub name: String,
    pub cpf: String,
    pub age: Option<i32>,
    //published: Bool,
}


impl Contact {
    fn from(contact: Contact) -> Contact {
        Contact {
            id: 0,
            name: contact.name,
            cpf: contact.cpf,
            age: contact.age,
        }
    }
}

// impl Contact {
//     pub fn new(id: i32, name: String, cpf: String, age: i32, published: Bool) -> Self {
//         Contact { id, name, cpf, age, published }
//     }
// }