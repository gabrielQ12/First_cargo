
fn main() {
    let mut x = vec!["Hello", "world"];
    
    x.push("Aha"); // Hello world Aha
    x.insert(1, "to the"); // Hello to the world Aha
    x.remove(1); // Hello world Aha
    x.pop(); //Hello world
    for i in 0..x.len() { println!("{}", x[i]);}
    
}  