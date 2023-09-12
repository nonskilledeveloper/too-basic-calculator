pub fn sumar(a: i32, b: i32) -> i32 {
    let resultado = a+b;
    println!("La suma es: {}", resultado);
    return resultado
}

pub fn restar(a: i32, b: i32) -> i32 {
    let resultado = a-b;
    println!("La resta es: {}", resultado);
    resultado
}

pub fn multiplicar(a: i32, b: i32) -> i32 {
    let resultado = a*b;
    println!("La multiplicación es: {}", resultado);
    resultado
}

pub fn dividir(a: i32, b: i32) -> f64 {
    let a_flt: f64 = a as f64;
    let b_flt: f64 = b as f64;

    let resultado = a_flt/b_flt;
    println!("La división es: {}", resultado);
    return resultado
}
