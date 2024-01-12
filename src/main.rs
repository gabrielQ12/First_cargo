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
        let bar = bar-6; //5
        println!("La valeur du bar a l'interieur acolade est : {bar}.");

    }
     //11
    println!("La valeur de bar est : {bar}.");
    
}
