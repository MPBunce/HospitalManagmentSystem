use actix_web::{web, HttpResponse, Responder};
use crate::db::AppState;

//Models
use crate::models::*;

async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

pub async fn get_all_physicians(db: web::Data<AppState>) -> impl Responder {

    let conn = db.db_pool.get().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM Physician").unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(Physician {
            employee_id: row.get(0)?,
            name: row.get(1)?,
            position: row.get(2)?,
            ssn: row.get(3)?,
        })
    }).unwrap();

    let physicians: Vec<Physician> = rows.map(|r| r.unwrap()).collect();

    HttpResponse::Ok().json(physicians)

}

pub async fn get_nurses(db: web::Data<AppState>) -> impl Responder {

    let conn = db.db_pool.get().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM Nurse").unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(Nurse {
            employee_id: row.get(0)?,
            name: row.get(1)?,
            position: row.get(2)?,
            registered: row.get(3)?,
            ssn: row.get(4)?,
        })
    }).unwrap();

    let nurses: Vec<Nurse> = rows.map(|r| r.unwrap()).collect();

    HttpResponse::Ok().json(nurses)

}


pub async fn get_physician(db: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {

    let conn = db.db_pool.get().unwrap();
    let statement_string = "SELECT * FROM Physician WHERE EmployeeID = ".to_owned() + &path.to_string();
    let mut stmt = conn.prepare( &statement_string ).unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(Physician {
            employee_id: row.get(0)?,
            name: row.get(1)?,
            position: row.get(2)?,
            ssn: row.get(3)?,
        })
    }).unwrap();

    let physicians: Vec<Physician> = rows.map(|r| r.unwrap()).collect();

    HttpResponse::Ok().json(physicians)

}


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/").route(web::get().to(hello_world)))
        .service(web::resource("/api/get_all_physicians").route(web::get().to(get_all_physicians)))
        .service(web::resource("/api/get_nurses").route(web::get().to(get_nurses)))
        .service(web::resource("/api/get_physician/{id}").route(web::get().to(get_physician)))
    ;

}