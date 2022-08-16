//#![allow(unused_variables)]

use std::collections::HashMap;

fn main() {
    let mut flights = HashMap::new();
    
    flights.insert("K1423", ("11:12", "Orlando"));
    flights.insert("K1123", ("12:05", "Salt Lake City"));
    flights.insert("K441", ("09:43", "London"));
    flights.insert("K21", ("06:20", "Boston"));
    flights.insert("K541", ("15:30", "Berlin"));
    flights.insert("K1122", ("17:11", "Nashville"));

    let  flight_number = "K21";

    let option = flights.get(flight_number);
    let (time, destination) = option.unwrap();
    println!("{} {}", time, destination);
}
