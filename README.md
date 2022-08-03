
Organización de lenguajes y compiladores 2

* # Instrucción Print             
    El enunciado menciona que solo se puede usar println!. No tomar en cuenta el uso de print!

* # Usize                
    El uso de este tipo de dato se restringe al uso de vectores, cuando se utiliza la función with_capacity o se compara el tamaño de un vector https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len


* # Concatenacion           

Para concatenar strings, la primera expresión debe ser de tipo String y la segunda de tipo &str 

```rust


fn main() {
    let mut string1: String = "hola".to_owned();
    let string2: &str = "mundo";
    
    let string3 = string1 + string2;
    
    println!("{}",string3);
    println!("{}",string1); 
}

error[E0382]: borrow of moved value: `string1`
  --> main.rs:17:19
   |
11 |     let mut string1: String = "hola".to_owned();
   |         ----------- move occurs because `string1` has type `String`, which does not implement the `Copy` trait
...
14 |     let string3 = string1 + string2;
   |                   ----------------- `string1` moved due to usage in operator
...
17 |     println!("{}",string1);
   |                   ^^^^^^^ value borrowed here after move
   |
note: calling this operator moves the left-hand side
  --> main.rs:14:19
   |
14 |     let string3 = string1 + string2;
   |                   ^^^^^^^^^^^^^^^^^
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error; 1 warning emitted
For more information about this error, try `rustc --explain E0382`.


```

rust mueve el valor de string1 al hacer la concatenación, lo que mostraría un error, por lo que para evitar ese error se puede usar la función clone() sobre string1. 



```rust


fn main() {
    let mut string1: String = "hola".to_owned();
    let string2: &str = "mundo";
    
    let string3 = string1.clone() + string2;
    
    println!("{}",string3);
    println!("{}",string1); 
}

```

* # Funciones nativas
```rust
fn main() {
    
    let dato: f64 = 5.5;
    let raiz = (dato).sqrt();
    let absoluto = (dato).abs();
    let cadena = (dato).to_string();
    let copia = (dato).clone();
    println!("{}", raiz);
    println!("{}", absoluto);
    println!("{}", cadena);
    println!("{}", copia);
    
}
```
