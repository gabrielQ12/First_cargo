
#[allow(dead_code)]

enum Continent {
    Africa,
    Asia,
    Europe,
    NorthAmerica,
    Oceania,
    SouthAmerica,
}

#[allow(dead_code)]
enum CardinalPoint {
    North,
    South,
    East,
    West,
}

fn main() {
    let continent = Continent::Europe;

    match continent  {
        Continent::Europe => println!("EU"),
        Continent::Africa => println!("AF"),
        Continent::Asia => println!("AS"),
        Continent::NorthAmerica => println!("NA"),
        Continent::Oceania => println!("OC"),
        Continent::SouthAmerica => println!("SA"),
    }

    let c_point = CardinalPoint::North;

    match c_point {
        CardinalPoint::North => println!("N"),
        CardinalPoint::South => println!("S"),
        CardinalPoint::East => println!("E"),
        CardinalPoint::West => println!("W"),
    }

}

