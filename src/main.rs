
fn find_something<T : std::fmt::Display>(something: T) {
    println!("Votre valeur est: {}", something);
}



fn main() {
    find_something(7);
    find_something(String::from("value"));
    

}