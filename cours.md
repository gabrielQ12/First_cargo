 ```Rust
 fn main() {
     let mut foo =10;
     println!("La valeur de foo est : {foo}.");
     foo = 20;
     println!("La valeur de foo est : {foo}.");

     const FOO_BAR: i32 = 10;
     println!("La valeur de FOO_BAR est : {FOO_BAR}.");

     let bar = 10;
     let bar = bar +1;

     {
         let bar = bar-6; 5
         println!("La valeur du bar a l'interieur acolade est : {bar}.");

     }
      11
     println!("La valeur de bar est : {bar}.");
    
 }
```
-------------

```Rust
 fn main() {
      println!("{}, {}", "Hello", "world!");
     let x =10;
     let y = 5;

     println!("{}",x + y ); //montre seulement le resultat
     println!("{} + {} = {}",x, y, x + y ); //montre toute l'opération
     println!("{} - {} = {}",x, y, x - y );
     println!("{} * {} = {}",x, y, x * y );
     println!("{} / {} = {}",x, y, x / y );
     println!("{} % {} = {}",x, y, x % y );

 }
```
-------------

```Rust
 fn main () {
    //scalar types : https://doc.rust-lang.org/std/index.html#primitives
    //integers, floating-point, booleans, characters
    
    //integer
     // 8, 16, 32, 64, 128 
     let x: i8 =5;
     let y: i8 = 10;

     //10: 98_128
     //16 (hexadecimal): 0x15f
     //8 (octal) : 0o123
     //2 (binaire) : 0b1111_0000

     println!("{}",x-y);

      //loating point
     let dec = 2.5;
     println!("{}", dec);

     //booleans
     let boolean = true;
     println!("{}",boolean);

     //characters
     let ch = 'z';
     println!("{}",ch);

    
 }
```
-------------

```Rust
 fn main() {

      bool = true/false
     // ==    !=     <  <=  >    >=

      let x = 5;
      let y = 10;

      println!("{}", x == y); false
      println!("{}", x != y); true
      println!("{}", x < y); true
      println!("{}", x <= y); true
      println!("{}", x > y); false
      println!("{}", x >= y); false

     println!("{}", "abc" < "abcd"); 

    //  + - * / % 
     let mut z = 10;
     z += 2;  //c'est une opération et une assignation a la fois z = z+1
     z -=4;
     z *= 2;
     z /= 4;

     println!("{}", z);

 }
```
-------------

```Rust

fn main() {
    let x = -5;
    let condition = x > 0;
    
    if x > 0 { 
        println!("Le nombre est positif ! ");
    } else {
        println!("Le nombre est négatif ! ");
    }
    println!("{}", condition);
}
```
-------------

```Rust
fn main() {
    let age = 17;
    
    if age < 18 { 
        println!("Jean-Eude est mineur");

    } else if age > 18 {
        println!("Jean-Eude est majeur");
        
    } else {
        println!(" Jean-Eude viens d'atteindre la majorité")
    }
    
}

```
-------------

```Rust

fn main() {
    let x = 5;
    let y = if x > 0  { 5 } else { 6 };
    
    println!("{}", y);
}
```
-------------

```Rust

fn main() {
    let mut x =0;

    //while ==> boucle tant que 

    while x < 20 {
        x +=1;
        if x %3 != 0 { continue; }
        if x * x > 200 { break ; }
        println!("{}", x);
    }
}
```
-------------

```Rust

fn main() {
    let mut x =0;

    loop {
        if x * x > 200 { break; }
        println!("{}", x);
        x +=1;
    }
// loop est identique que un while true,  cela ferai une boucle infinie ,  il faut mettre une condition de break

}
```
-------------

```Rust

fn main() {
    let mut x =0;

    for x in 1..11 {
        if x == 5 { break; }
        println!("{}", x);
    }
} 
```
-------------

```Rust

fn main() {
    let mut x =0;

    for x in 1..11 {
        if x == 5 { break; }
        println!("{}", x);
    }
} 
```
-------------

```Rust

fn main() {
   let mut x = ["This", "world", "displays", "hello"];

   println!("{} {} {} {}.", x[0], x[2], x[3], x[1]);
   println!("La taille de l'array est : {}.", x.len()); //len ==> length

   x[1] = "Alex";
   println!("{} {} {} {}.", x[0], x[2], x[3], x[1]);

   let mut y = ["Foo", "Alex"];
   y[1] = "bar";
   println!("{} {}.", y[0], y[1]);

   let mut z = [3; 15];
   println!("La taille de l'array est : {}.", z.len()); //len ==> length
   z[10] = 50;
   println!("{} {}.", z[5], z[10]);

   for i in 0..z.len(){
   println!("index {}: {} " , i, z[i]);
} 
}

```
-------------

```Rust

fn main() {
    let mut x = vec!["Hello", "world"];
    println!("Length {}!", x.len());
    println!("{},{}!", x[0], x[1]);

    x.push("aha.");
    println!("Length {}!", x.len());
    println!("{},{}! {}.", x[0], x[1], x[2]);

}

```
-------------

```Rust
fn main() {
    let mut x = vec!["Hello", "world"];
    
    x.push("Aha"); // Hello world Aha
    x.insert(1, "to the"); // Hello to the world Aha
    x.remove(1); // Hello world Aha
    x.pop(); //Hello world
    for i in 0..x.len() { println!("{}", x[i]);}
    
}  
```
-------------

```Rust

fn main() {
    let x = vec![1, 2, 3];
    let mut y = vec![0;0];
    
    println!("Vec X:{:?}, Vec Y: {:?}", x,y);

    y=x;

    println!("Vec Y: {:?}",y);


}  
```
-------------

```Rust

fn main() {
   let x = {
    let y =3;
    y+1
   }; 

   print!("La valeur de X est : {}", x);
}  
```
-------------

```Rust

fn main() {
   
   let x = foo_fonction(30);
   println!("La valeur de X est {}", x);

}  


fn foo_fonction(x: i32) -> i32 {
    x
}
```
-------------

```Rust

fn main() {
    let tup = (5, 3.14, true);
    let (x, y, z) = tup;
    

    println!("{x} {y} {z}");
}
```
-------------

```Rust

fn main() {
    //#[derive(Debug)]
    struct Dog{
        name: String,
        age: i32,
        gender: String,
        under_10: bool
    }

    let first_dog = Dog{
        name: String::from("Woof"),
        age: 8,
        gender: String::from("Male"),
        under_10: true
    };

    println!("Mon chien s'appelle {}, il a {} ans, c'est un {} et il a moins de 10 ans,  c'est {}", 
    first_dog.name,first_dog.age,first_dog.gender,first_dog.under_10);    
}
```
-------------

```Rust

struct Dog{
    name: String,
    age: i32,
    

}


fn main() {
    //#[derive(Debug)]
    let first_dog = register_dog(String::from("Woof"),8);
    println!("Mon chien s'appelle {}, il a {} ans.", 
    first_dog.name,first_dog.age); 
}
fn register_dog(name:String,age:i32) -> Dog {
    Dog{
        name: name,
        age: age
    }
}
```
-------------

```Rust

fn main() {
    println!("\t Tabulation.  \n Nouvelle ligne !  c:\\node\\link ");
    println!(r#" Tabulation.  Nouvelle ligne !  c:\node\Link "#); 
    println!("{:#?}", b" \t Tabulation.  \n Nouvelle ligne !"); 
    println!("{:X}", 'H' as u32);
    println!("\u{48}");

    let x = 50;
    println!("{}", x);
    println!("{x}");
    println!("{0}",x);

     let y = 9;
     let y_ref=&y;
     println!("{:p}", y_ref);

     let name = "Alex";
     println!("{name:-^12}");
}
```
-------------

```Rust
fn main() {
   // let mut x = String::from ("hello");
   // x.push_str(", world ! ");

   // println!("{x}");

   // {
    // let foo = String::from("Hello");
   // }



   // &str / String

   // &str est immuable il ne peut pas etre modifié (accès a la memoir rapide et efficace)
        // la mémoire est allouée a la compilation et est stockée dans le binaire
        // (pas besoin de gérer la mémoire nous même)

    // --- //

   // String est mutable (accès a la memoir plus lent)
        // la mémoire doit etre demander a l'allocateur ,  nous devons gérer la mémoire nous même 
        // L'utilisation de String::from permet de demander la mémoire a l'allocateur
        // quand nous refermons le scoop la mémoire est libérée automatiquement

        let  x = String::from ("Alex");
        let y = x.clone();

        println!("{x} {y}");

        let foo =5;
        let bar = foo;

        println!("{foo} {bar}");

}
```
-------------

```Rust
fn main() {
    let x = String::from("Alex");
    let x_len = calculate_length(&x);
    println!("Length de {x} est : {x_len}!");
}

fn calculate_length(s: &String) -> usize {
    s.len()

    // ici x ne peu pas etre allouer une second fois,  il est donc passé en référence grace a l'utilisation de &
    // la fonction calculate_length prend donc une référence de x
    // la fonction ne prend pas possession de x
    // la fonction ne peut pas changer x
    // la fonction ne peut pas libérer x
    // la fonction ne peut pas faire quoi que ce soit qui pourrait invalider x

    // l'owner est toujours x , et la fonction ne fait que l'emprunter

    // créer une référence en Rust s'apelle le "borrowing"
    // le "borrowing" permet de passer une référence à une fonction plutot que de passer la valeur
    // cela permet de ne pas avoir à copier de grandes quantités de données
    // cela permet de ne pas avoir à donner la propriété de ces données à une autre variable
    // cela permet de ne pas avoir à libérer ces données lorsque la fonction a terminé
    // cela permet de ne pas avoir à penser à la manière dont ces données pourraient être utilisées après que la fonction a terminé
    

}
```
-------------

```Rust

fn main() {
    let mut x = String::from("Alex");
    println!("{x}");
    modify_borrow(&mut x);
    println!("{x}");
}

fn modify_borrow(s: &mut String) {
    s.push_str(", c'est mon nom");

    // une référence est immuable
    // on ne peut pas modifier la valeur d'une référence
    // pour modifié une référence il faut ajouter &mut
    // il est impossible de créer deux reference modifiable sur une seul valeur 
    
}
```
-------------

```Rust

fn main() {
    let bar = foo();
    println!("{bar}");
}

fn foo() -> String {
    let s = String::from("Alex");
    s
}
```

-------------

```Rust
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
```

-------------

```Rust


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


```

-------------

```Rust

```

-------------

```Rust

```

-------------

```Rust

```

-------------

```Rust

```