use std::io;
mod calcular;

fn main() {
    let mut seleccion = String::new();
    let mut a = String::new(); 
    let mut b = String::new();

    println!(r"
 ____ ____ ____ ____ ____ ____ 
||B |||a |||l |||d |||o |||r ||
||__|||__|||__|||__|||__|||__||
|/__\|/__\|/__\|/__\|/__\|/__\|        

1. Sumar
2. Restar
3. Multiplicar
4. Dividir
");

    println!("Selecciona qué deseas realizar: ");
    io::stdin()
        .read_line(&mut seleccion)
        .expect("Error");

    println!("Escribe el primer número: ");
    io::stdin()
        .read_line(&mut a)
        .expect("Error");

    println!("Escribe el segundo número: ");
    io::stdin()
        .read_line(&mut b)
        .expect("Error");

    // Convertir a números

    let seleccion: i8 = seleccion.trim().parse().expect("Error");
    let a: i32 = a.trim().parse().expect("Error");
    let b: i32 = b.trim().parse().expect("Error");

    // También se puede de la siguiente manera:
    // let a: i32 = a.parse()

    if seleccion == 1 {
        calcular::sumar(a, b);
    } else if seleccion == 2 {
        calcular::restar(a, b);
    } else if seleccion == 3 {
        calcular::multiplicar(a, b);
    } else if seleccion == 4 {
        calcular::dividir(a, b);
    }
}
