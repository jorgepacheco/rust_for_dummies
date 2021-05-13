fn main() {

    println!("Hello2, world!!!!!");
    variables();
    operaciones();
}


fn variables(){
    // se definen con let nombre : tipo(opcional) = valor;
    let age : u16 = 47;
    // Interpolación
    println!("I'm Jorge i am {} years old", age);

    // Variables mutables
    let mut variable_mutable : u8 = 1;
    println!("variable_mutable init value {}", variable_mutable);

    variable_mutable = 23;
    println!("variable_mutable changed {}", variable_mutable);

}

fn operaciones() {

        // Addition
        println!("1 + 2 = {}", 1u32 + 2);

        // Subtraction
        println!("1 - 2 = {}", 1i32 - 2);
        // ^ Try changing `1i32` to `1u32` to see why the type is important

        // Integer Division
        println!("9 / 2 = {}", 9u32 / 2);

        // Float Division
        println!("9 / 2 = {}", 9.0 / 2.0);

        // Multiplication
        println!("3 * 6 = {}", 3 * 6)

}