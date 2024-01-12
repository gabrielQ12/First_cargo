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
 ``````
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
 ``````
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

 ```Rust

fn main() {
    let x = 5;
    let y = if x > 0  { 5 } else { 6 };
    
    println!("{}", y);
}
  ```

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

   ```Rust

fn main() {
    let mut x =0;

    for x in 1..11 {
        if x == 5 { break; }
        println!("{}", x);
    }
} 
  ```

    ```Rust

  ```

    ```Rust

  ```

    ```Rust

  ```

    ```Rust

  ```

    ```Rust

  ```

    ```Rust

  ```