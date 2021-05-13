# RUST_FOR_DUMMIES

- Documentation https://www.rust-lang.org/
- Install https://rustup.rs/
- Tutorial: https://docs.microsoft.com/es-es/learn/modules/rust-understand-common-concepts/

# Resources

https://www.youtube.com/watch?v=AD1JyZ3blgI&t=0s

# Data Types

Rust es un lenguaje con tipos estáticos, lo que significa que el compilador debe conocer exactamente los tipos de datos de cada variable del código.

```rust
let number: u32 = "42".parse().expect("Not a number!");
```

```rust
let number = "42".parse().expect("Not a number!");
```

Error ya que no hemos indicado el tipo

- Tipos numericos: Los 'i' con signo y los 'u' positivos

8  bits  i8   u8
16 bits  i16  u16
32 bits  i32  u32
64 bits  i64  u64
128 bits i128 u128
arco     isize    usize

- Los float usan la 'f'

```rust
let x = 2.0;      // f64, default type
let y: f32 = 3.0; // f32, via type annotation
```

- Operaciones aritmeticas básicas
```rust
fn main() {
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
```

