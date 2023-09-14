//Crie uma função que receba dois números inteiros como parâmetros e retorne a soma deles.

fn inteiros(a: i32, b: i32)->i32{
    a + b 
}

fn main() {
    let soma: i32 = inteiros(2, 5);
    println!("O valor é {}", soma);
}
