
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