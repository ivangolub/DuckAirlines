//#![allow(nonstandard_style)]
use std::vec;
const EARTH_RADIUS_IN_KM: f64 = 6371.0;
#[derive (Copy, Clone)]
struct Location <'a>
{
    name: &'a str,
    latitude: f64,
    longitude: f64 
}

struct Route<'a>
{
    airports: Vec<Location<'a>>
}

impl Location <'_>
{
    fn new(name: & str, latitude: f64, longitude: f64 ) -> Location
    {
        Location
        {
            name,
            latitude, 
            longitude,
        }
    }
}

impl Route<'_>
{
    fn new(airports: Vec<Location>) -> Route
    {
        Route
        {
            airports
        }
    }

    fn total_distance(airports: Vec<Location>) -> f64
    {
        let mut total_distance = 0.0;
        let mut previous_waypoint: Option<Location> = None;

        for waypoint in airports.iter()
        {
            match previous_waypoint {
                None => {
                    previous_waypoint = Option::from(waypoint.clone());
                    continue;
                }
                Some(previous_waypoint_value) => {
                    let previous_waypoint_radians = previous_waypoint_value.latitude.to_radians();
                    let waypoint_radians = waypoint.latitude.to_radians();
    
                    let delta_latitude = (previous_waypoint_value.latitude - waypoint.latitude).to_radians();
                    let delta_longitude = (previous_waypoint_value.longitude - waypoint.longitude).to_radians();
    
                    let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
                        + previous_waypoint_radians.cos()
                            * waypoint_radians.cos()
                            * f64::powi((delta_longitude / 2.0).sin(), 2);
    
                    let central_angle = 2.0 * inner_central_angle.sqrt().asin();
                    let distance = EARTH_RADIUS_IN_KM * central_angle;
    
                    total_distance += distance;
    
                    previous_waypoint = Option::from(waypoint.clone());
    
                    println!(
                        "The distance between {} and {} is {:.1} kilometers",
                        previous_waypoint_value.name, waypoint.name, distance
                    );
                }
            }
        }
        total_distance
    }
}


fn main() {
    let airports: Vec<Location> = vec![
         Location::new("KCLE", 41.4075, -81.851111),
         Location::new("LEYIR", 41.51030, -83.88080),
         Location::new("PIONS", 41.65390, -84.48190),
         Location::new("ZOSER", 41.72390, -84.78130),
         Location::new("MODEM", 41.72800, -84.89730),
         Location::new("BRYTO", 41.74170, -85.31320),
         Location::new("SEWTO", 41.74780, -85.51130),
         Location::new("GIJ", 41.76860, -86.31850),
         Location::new("NEPTS", 41.96750, -87.05300),
         Location::new("THORR", 42.12330, -87.60030),
         Location::new("OBK", 42.22140, -87.95160),
         Location::new("COTON", 42.31990, -89.31220),
         Location::new("DBQ", 42.40150, -90.70910),
         Location::new("VIGGR", 42.55520, -93.12410),
         Location::new("FOD", 42.61110, -94.29480),
         Location::new("ONL", 42.47050, -98.68690),
         Location::new("BFF", 41.89420, -103.48200),
         Location::new("OCS", 41.59020, -109.01500),
         Location::new("PUDVY", 41.54270, -109.34200),
         Location::new("WEGEM", 41.44560, -109.99000),
         Location::new("KSLC", 40.7861, -111.9822)
    ];

    let route77: Route = Route::new(airports);

    println!(
        "\nThe total distance between the two points is {:.1} kilometers",
        Route::total_distance(route77.airports)
    );

}
