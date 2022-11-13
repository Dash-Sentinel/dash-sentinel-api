CREATE TABLE sightings (
    id SERIAL PRIMARY KEY,
    car_color VARCHAR,
    car_plate VARCHAR,
    car_make VARCHAR,
    car_model VARCHAR, 
    car_year VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    geog GEOGRAPHY
)

INSERT INTO sightings VALUES (
    DEFAULT,
    'Red',
    'UTD 1969',
    'Ford',
    'Focus',
    '1998',
    DEFAULT,
    'POINT(-118.4079 33.9434)'
)
