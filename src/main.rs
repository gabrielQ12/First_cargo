#[derive(Debug)]

struct Triangle {
    base: u32,
    side: u32,
}

fn main() {
    // Triangle isocèle : base + (2 * coté)
    // base : 8cm, coté : 10 cm -> 8+(2*10) = 28 cm
    // x = base , y = coté
    let triangle = Triangle {
        base: 8,
        side: 10,
    };

    println! (
        "Notre triangle : {:?} \nLe périmètre d'un triangle isocèle de {}cm et de  {}cm est égal à :{}cm",
        triangle, triangle.base, triangle.side, calculate_perimetre(&triangle)
    );
}

fn calculate_perimetre(triangle: &Triangle) -> u32 {
    triangle.base + (2* triangle.side)
}