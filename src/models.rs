//use diesel::{sql_types::Bool};
use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

use crate::schema::contacts; // Importe o módulo schema para acessar a tabela contacts




#[derive(Debug, Insertable)]
#[table_name = "contacts"]
pub struct NewContact {
    pub name: String,
    //pub cpf: String,
    //pub age: i32,
   //pub published: bool,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Contact {
    //#[diesel(sql_type = Integer)]
    pub id: i32,
    //#[diesel(sql_type = Text)]
    pub name: String,
    //#[diesel(sql_type = Text)]
    pub cpf: String,
    //#[diesel(sql_type = Integer)]
    //age: i32,
    //#[diesel(sql_type = Bool)]
    //published: Bool,

    
}

// impl Contact {
//     pub fn new(id: i32, name: String, cpf: String, age: i32, published: Bool) -> Self {
//         Contact { id, name, cpf, age, published }
//     }
// }