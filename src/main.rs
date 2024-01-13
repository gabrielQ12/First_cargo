
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