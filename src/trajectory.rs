pub use crate::checkpoint::Location;
    pub struct Route<'a>
    {
        pub airports: Vec<Location<'a>>
    }

    impl Route<'_>
    {
        pub fn new(airports: Vec<Location>) -> Route
        {
            Route
            {
                airports
            }
        }

        pub fn total_distance(airports: Vec<Location>) -> f64
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
                        let distance = crate::checkpoint::EARTH_RADIUS_IN_KM * central_angle;
                    
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