//#![allow(unused_variables)]

fn main() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let route = [
        ("Hanko", 59.80890879537541, 22.892719517792777),
        ("Nuorgam", 70.08970598971996, 27.950602372944243),
        ("Hanko", 59.80890879537541, 22.892719517792777)
    ];

    let mut total_distance = 0.0;
    let mut previous_waypoint: Option<(&str, f64, f64)> = None;

    for waypoint in route.iter() {
        match previous_waypoint {
            None => {
                previous_waypoint = Option::from(waypoint.clone());
                continue;
            }
            Some(previous_waypoint_value) => {
                let previous_waypoint_radians = previous_waypoint_value.1.to_radians();
                let waypoint_radians = waypoint.1.to_radians();

                let delta_latitude = (previous_waypoint_value.1 - waypoint.1).to_radians();
                let delta_longitude = (previous_waypoint_value.2 - waypoint.2).to_radians();
            
                let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
                    + previous_waypoint_radians.cos()
                    * waypoint_radians.cos()
                    * f64::powi((delta_longitude / 2.0).sin(), 2);
                let central_angle = 2.0 * inner_central_angle.sqrt().asin();
            
                total_distance += central_angle;
                previous_waypoint = Option::from(waypoint.clone());
            }
        }
    }

    println!("The distance between the two points is {:.1} kilometers", EARTH_RADIUS_IN_KILOMETERS * total_distance);
}
