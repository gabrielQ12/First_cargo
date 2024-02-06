

struct Triangle {
    base: u32,
    side: u32,
}

impl Triangle {
    fn calculate_perimetre(&self) -> u32 {
        self.base + (2* self.side)
    }

    fn is_base_greater_then_side(&self) -> bool {
        self.base > self.side
    }

    fn is_triangle_bigger(&self, other: &Triangle) -> bool {
        self.calculate_perimetre() > other.calculate_perimetre()
    }
}

fn main() {
    // first_triangle isocèle : base + (2 * coté)
    // base : 8cm, coté : 10 cm -> 8+(2*10) = 28 cm
    // x = base , y = coté
    let first_triangle = Triangle {
        base: 15,
        side: 12
    };

    let second_triangle = Triangle {
        base:22,
        side:25
    };



    println! (
        "Le périmètre d'un first_triangle isocèle de {}cm et de  {}cm est égal à :{}cm",
        first_triangle.base, first_triangle.side, first_triangle.calculate_perimetre()
    );

    println!(
        "La base est-elle plus grande que le côté ? : {}",
        first_triangle.is_base_greater_then_side()
    );

    println!(
        "La base est-elle plus grande que le côté ? : {}",
        second_triangle.is_base_greater_then_side()
    );

    println!(
        " Le premier triangle est il plus grand que le second ? :{}  ({} > {})",
        first_triangle.is_triangle_bigger(&second_triangle), first_triangle.calculate_perimetre(), second_triangle.calculate_perimetre()
    );
}

