use std::{
        fs::File, 
        io::Read,
    };

use day3::{find_coordinates, match_coordinates, string_to_wire, closest_intersection, Wire};
use futures::*;

fn main() {
    
    let mut wire_maps = String::new();
    let _f = File::open("input.txt")
        .unwrap()
        .read_to_string(&mut wire_maps);

    let wire_maps: Vec<String> = wire_maps
        .lines()
        .map(|wire| wire.to_string())
        .collect();

    let wire_1 = wire_maps
        .get(0)
        .unwrap()
        .split(',')
        .map(|wire| string_to_wire(wire))
        .collect::<Vec<Wire>>();

    let wire_2 = wire_maps
        .get(1)
        .unwrap()
        .split(',')
        .map(|wire| string_to_wire(wire))
        .collect::<Vec<Wire>>();        

    let wire_1 = find_coordinates(wire_1);
    let wire_2 = find_coordinates(wire_2);

    

    let mut crossing_coordinates = match_coordinates (wire_1, wire_2);

    crossing_coordinates.remove(0);

    if crossing_coordinates.is_empty() {
        println!("There is no answer!");
        panic!()
    }

    let answer = closest_intersection (crossing_coordinates);
    
    println!("{:?}", answer);

}


