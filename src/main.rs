#[macro_use]
extern crate rocket;

use rocket::{
    fairing::{self, AdHoc},
    serde::json::Json,
    Build, Rocket,
};
use rocket_db_pools::{sqlx::Row, Connection, Database};
use serde::{Deserialize, Serialize};
use sqlx::query;

#[derive(Database)]
#[database("postgis")]
struct Postgis(sqlx::PgPool);

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Postgis::fetch(&rocket) {
        Some(db) => match sqlx::migrate!().run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

fn db_fairing() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Postgis::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
    })
}

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
    rocket::build().attach(db_fairing()).mount(
        "/",
        routes![
            healthcheck,
            get_alerts,
            get_sitings,
            get_sitings_within_radius
        ],
    )
}
