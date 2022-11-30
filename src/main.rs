//#![allow(nonstandard_style)]
use std::vec;

pub mod checkpoint;
pub mod trajectory;

use checkpoint::Location;
use trajectory::Route;
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
