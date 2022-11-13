#[macro_use]
extern crate rocket;

use serde::Serialize;
use rocket::serde::{json::Json};
use rocket_db_pools::{
    sqlx::Row,
    Connection, Database,
};

#[derive(Database)]
#[database("postgis")]
struct Postgis(sqlx::PgPool);

#[get("/alerts")]
async fn get_alerts(mut db: Connection<Postgis>) -> String {
    match sqlx::query(
        "SELECT json_build_object(
            'id',          id,
            'person_name', person_name,
            'gender',      gender,
            'race',        race,
            'age',         age,
            'car_color',   car_color,
            'car_plate',   car_plate,
            'car_make',    car_make,
            'car_model',   car_model,
            'car_year',    car_year,
            'created_at',  created_at,
            'location',    location,
            'geometry',    ST_AsGeoJSON(geog)::json
        ) FROM alerts;",
    )
    .fetch_all(&mut *db)
    .await
    {
        Ok(data) => format!("[{}]", data
            .iter()
            .map(|r| format!("{}", r.get_unchecked::<String, _>(0)))
            .collect::<Vec<String>>()
            .join(",")),
        _ => String::from(""),
    }
}

#[get("/sightings")]
async fn get_sightings(mut db: Connection<Postgis>) -> String {
    match sqlx::query(
        "SELECT json_build_object(
            'id',          id,
            'car_color',   car_color,
            'car_plate',   car_plate,
            'car_make',    car_make,
            'car_model',   car_model,
            'car_year',    car_year,
            'created_at',  created_at,
            'geometry',    ST_AsGeoJSON(geog)::json
        ) FROM sightings;"
    )
    .fetch_all(&mut *db)
    .await
    {
        Ok(data) => data 
            .iter()
            .map(|r| format!("{}", r.get_unchecked::<String, _>(0)))
            .collect::<Vec<String>>()
            .join(""),
        _ => String::from("")
    }
}

#[get("/sightings-within-radius?<long>&<lat>&<radius>")]
async fn get_sightings_within_radius(mut db: Connection<Postgis>, long: f64, lat: f64, radius: f64) -> Json<impl Serialize> {
    let records =  sqlx::query!(
        "SELECT json_build_object (
            'id',          id,
            'car_color',   car_color,
            'car_plate',   car_plate,
            'car_make',    car_make,
            'car_model',   car_model,
            'car_year',    car_year,
            'created_at',  created_at,
            'geometry',    ST_AsGeoJSON(geog)::json
        ) FROM sightings WHERE ST_Distance(geog, ST_MakePoint($1 , $2)) <= $3;", long, lat, radius
    )
    .fetch_all(&mut *db)
    .await.unwrap()
    .iter()
    .map(|r| r.json_build_object.as_ref().unwrap())
    .cloned()
    .collect::<Vec<_>>();
    Json(records)
}

#[get("/healthcheck")]
fn healthcheck() -> String {
    String::from("API is online!")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Postgis::init())
        .mount("/", routes![healthcheck, get_alerts, get_sightings, get_sightings_within_radius])
}
