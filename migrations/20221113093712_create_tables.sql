CREATE TABLE IF NOT EXISTS sightings (
    id SERIAL PRIMARY KEY,
    car_color VARCHAR,
    car_plate VARCHAR,
    car_make VARCHAR,
    car_model VARCHAR, 
    car_year VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    geog GEOGRAPHY
);

CREATE TABLE IF NOT EXISTS alerts ( 
    id SERIAL PRIMARY KEY, 
    person_name VARCHAR, 
    gender VARCHAR, race 
    VARCHAR, age INT, 
    car_color VARCHAR, 
    car_plate VARCHAR, 
    car_make VARCHAR, car_model 
    VARCHAR, car_year VARCHAR, 
    created_at TIMESTAMP NOT NULL DEFAULT now(), 
    location VARCHAR, 
    geog GEOGRAPHY
);
