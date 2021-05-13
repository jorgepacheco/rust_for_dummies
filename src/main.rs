fn main() {

    println!("Hello, world!!!!!");
    variables()
}


fn variables(){
    // se definen con let nombre : tipo(opcional) = valor;
    let age : u16 = 47;
    // Interpolación
    println!("I'm Jorge i am {} years old", age);

    // Variables mutables
    let mut variableMutable : u8 = 1;
    println!("variableMutable init value {}", variableMutable);

    variableMutable = 23;
    println!("variableMutable changed {}", variableMutable);



}