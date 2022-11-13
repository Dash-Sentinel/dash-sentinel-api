CREATE TABLE alerts ( 
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

INSERT INTO alerts VALUES (     
    DEFAULT,     
    'John Doe',     
    'Male',     
    'Caucasian',     
    12,     
    'red',     
    'VADER',    
    NULL,    
    NULL,     
    '2022',     
    DEFAULT,     
    'Los Angeles',     
    'POINT(-118.4079 33.9434)' 
);