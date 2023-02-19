#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::{Serialize};
use rusqlite::{Connection, Result};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Person {
    key: String,
    user: String,
    contact: String,
    year: String,
    part: String,
    price: String,
    date: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
struct TempPerson {
    key: String,
    user: String,
    contact: String,
    year: String,
    part: String,
    price: String,
    date: String,
    url: String
}

fn mymain(year: String, make: String, model:String, part: String) -> Result<Vec<TempPerson>> {
    let conn = Connection::open("cars.db")?;
	
    let myquery: String = format!("SELECT NissanAltima.*, NissanAltimaPics.url FROM NissanAltima INNER JOIN NissanAltimaPics ON NissanAltima.key=NissanAltimaPics.key WHERE NissanAltima.part=:part AND NissanAltima.year=:year GROUP BY NissanAltima.key"); 

    let mut stmt = conn.prepare(&myquery)?;

    let person_iter = stmt.query_map(&[(":part",part.to_string().as_str()),(":year",year.to_string().as_str())], |row| {
        Ok(TempPerson {
            key: row.get(0)?,
            user: row.get(1)?,
            contact: row.get(2)?,
            year: row.get(3)?,
            part: row.get(4)?,
            price: row.get(5)?,
            date: row.get(6)?,
            url: row.get(7)?
        })
    })?;

    let mut my_vec: Vec<TempPerson> = vec![];

    for person in person_iter {
		let mydata = person.unwrap();
        my_vec.push(mydata);
	}
       
    let return_vec = my_vec.clone();
	Ok(return_vec) 
}
#[get("/cars/<year>/<make>/<model>/<part>")]
fn cars(year: String, make: String, model: String, part: String) -> Json<Vec<TempPerson>> {
    
    let output_vec = mymain(year, make, model,part);
    Json(output_vec.unwrap())
    
}

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/",routes![index,cars])
}