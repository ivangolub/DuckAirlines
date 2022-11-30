
pub const EARTH_RADIUS_IN_KM: f64 = 6371.0;
#[derive (Copy, Clone)]
pub struct Location <'a>
{
    pub name: &'a str,
    pub latitude: f64,
    pub longitude: f64 
}
impl Location <'_>
{
    pub fn new(name: & str, latitude: f64, longitude: f64 ) -> Location
    {
        Location
        {
            name,
            latitude, 
            longitude,
        }
    }
}