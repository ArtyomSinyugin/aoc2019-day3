use std::str::FromStr;

#[derive(PartialEq)] 
#[derive(Debug)]
pub struct Wire { 
    steps: i32,
    dir: Dir, 
}

impl FromStr for Wire {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let dir = chars
            .next()
            .map(|dir|{
                match Some(dir) {
                    Some('D') => Dir::D,
                    Some('U') => Dir::U,
                    Some('R') => Dir::R,
                    Some('L') => Dir::L,
                    Some(_) => panic!(),
                    None => panic!(),
                }
            })
            .unwrap();
        let steps: i32 = chars
            .collect::<String>()
            .trim()
            .parse::<i32>()
            .unwrap();
        
        Ok(Wire {steps, dir})
        
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Dir {
    R,
    L, 
    U, 
    D,
}

pub fn closest_intersection (crossing_coordinates: Vec<(i32, i32)>) -> i32 {
    let mut answer = crossing_coordinates[0].0.abs() + crossing_coordinates[0].1.abs();

    for i in 0..crossing_coordinates.len() {
    
        let (x, y) = crossing_coordinates[i];

        if answer > x.abs() + y.abs() {
            answer = x.abs() + y.abs();
        }
    }
    answer
}

pub fn match_coordinates (wire_1: Vec<(i32, i32)>, wire_2: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut x = Vec::new();

    'outer: for i in 0..wire_1.len() {
        if wire_1.get(i) == None {
            break;
        } else {
            for j in 0..wire_2.len () {
                if wire_1.get(i) == wire_2.get(j) {
                    if wire_2.get(j) == None {
                        break 'outer;
                    }
                    x.push(wire_1[i]);
                }
            }
        }
    }
    x
}

fn set_coordinates (direction: &Wire, xy_at_the_end_of_vector: (i32, i32)) -> Vec<(i32, i32)> {          // в функцию надо передать последние элементы вектора
    let (x, y) = xy_at_the_end_of_vector;
    match direction {
        Wire { steps, dir: Dir::D } => {
            //Vec::new();
            let vec = (1..=*steps)
                    .map(|step| (x, y - step))
                    .collect();
            vec
        },
        Wire { steps, dir: Dir::U } => {
            let vec = (1..=*steps)
                    .map(|step| (x, y + step))
                    .collect();
            vec
        },
        Wire { steps, dir: Dir::L } => {
            let vec = (1..=*steps)
                    .map(|step| (x - step, y))
                    .collect();
            vec
        },
        Wire { steps, dir: Dir::R } => {
            let vec = (1..=*steps)
                    .map(|step| (x + step, y))
                    .collect();
            vec
        },
    }
}


pub fn find_coordinates (v: Vec<Wire>) -> Vec<(i32, i32)> {
    let mut found_coordinates = vec![(0,0),];

    for i in 0..v.len() {
        let xy = found_coordinates[found_coordinates.len()-1];
        let mut vec = set_coordinates(v.get(i).unwrap(), xy);
        found_coordinates.append(&mut vec);
    }

    found_coordinates
}

pub fn string_to_wire (wire: &str) -> Wire {

    let wire: Wire = wire.parse().unwrap();

    wire
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_closest_intersection () {

        let wire = vec![(3, 4), (4, 7), (6, 10), (236, -365), (-1, 2), (256, 35), (-262, -358), (26, 35)];

        let answer = closest_intersection(wire);

        assert_eq!(3, answer);
    }

    #[test]
    fn check_match_coordinates () {
        let wire_2 = vec![(2, 3),(3, 4),(1, 7),(6, 10),(236, 365),(256, 35),(262, 358),(26, 35)];
        let wire_1 = vec![(22, 33),(6, 10),(11, 72),(26, 35)];

        let answer = match_coordinates(wire_1, wire_2);

        assert_eq!(vec![(6, 10),(26, 35)], answer);
    }

    #[test]
    fn check_from_str () {
        let s = "D23";
        let wire_1: Wire = s
            .parse()
            .unwrap();

        let wire_0 = Wire {
            steps: 23,
            dir: Dir::D,
        };

        assert_eq!(wire_0, wire_1);
    }

    #[test]
    fn check_set_coordinates () {
        let wire_down = Wire {
            steps: 3,
            dir: Dir::D,
        };
    
        let wire_up = Wire {
            steps: 2,
            dir: Dir::R,
        };

        let new = vec![(0,0),];

        let xy = new[new.len()-1];
    
        let mut vec1 = set_coordinates(&wire_down, xy);
    
        let xy = vec1[vec1.len()-1];
    
        let mut vec2 = set_coordinates(&wire_up, xy);
    
        vec1.append(&mut vec2);

        assert_eq!(vec![(0, -1), (0, -2), (0, -3), (1, -3), (2, -3)], vec1);

    }
}

/* решение задачи в асинхронности
pub fn closest_intersection (crossing_coordinates: Vec<(i32, i32)>) -> i32 {
    let mut answer = crossing_coordinates[0].0.abs() + crossing_coordinates[0].1.abs();

    for i in 0..crossing_coordinates.len() {
    
        let (x, y) = crossing_coordinates[i];

        if answer > x.abs() + y.abs() {
            answer = x.abs() + y.abs();
        }
    }
    answer
}

pub async fn match_coordinates (wire_1: Vec<(i32, i32)>, wire_2: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut x = Vec::new();
    println!("find_coordinates");
    'outer: for i in 0..wire_1.len() {
        if wire_1.get(i) == None {
            break;
        } else {
            for j in 0..wire_2.len () {
                if wire_1.get(i) == wire_2.get(j) {
                    if wire_2.get(j) == None {
                        break 'outer;
                    }
                    x.push(wire_1[i]);
                }
            }
        }
    }
    x
}

async fn set_coordinates (direction: &Wire, xy_at_the_end_of_vector: (i32, i32)) -> Vec<(i32, i32)> {          // в функцию надо передать последние элементы вектора
    //println!("set_coordinates");
    let (x, y) = xy_at_the_end_of_vector;
    match direction {
        Wire { steps, dir: Dir::D } => {
            //Vec::new();
            let vec = (1..=*steps)
                    .map(|step| (x, y - step))
                    .collect();
            vec
        },
        Wire { steps, dir: Dir::U } => {
            let vec = (1..=*steps)
                    .map(|step| (x, y + step))
                    .collect();
            vec
        },
        Wire { steps, dir: Dir::L } => {
            let vec = (1..=*steps)
                    .map(|step| (x - step, y))
                    .collect();
            vec
        },
        Wire { steps, dir: Dir::R } => {
            let vec = (1..=*steps)
                    .map(|step| (x + step, y))
                    .collect();
            vec
        },
    }
}


pub async fn find_coordinates (v: Vec<Wire>) -> Vec<(i32, i32)> {
    println!("find_coordinates");
    let mut found_coordinates = vec![(0,0),];

    for i in 0..v.len() {
        let xy = found_coordinates[found_coordinates.len()-1];
        let mut vec = set_coordinates(v.get(i).unwrap(), xy).await;
        found_coordinates.append(&mut vec);
    }

    found_coordinates
}

pub fn string_to_wire (wire: &str) -> Wire {

    let wire: Wire = wire.parse().unwrap();

    wire
}
*/