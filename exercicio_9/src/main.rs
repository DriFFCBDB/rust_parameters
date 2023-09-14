//Crie uma função que receba um número real (ponto flutuante) como parâmetro e retorne o valor absoluto desse número.

fn valor_absoluto(float_numero :f32) -> i32{

    let numero_arredondado = float_numero.round() as i32;
    let numero_absoluto = numero_arredondado.abs() as i32;
   
    numero_absoluto
}

fn main() {
    let float_numero = -2.29;
    let resultado = valor_absoluto(float_numero);
    println!("{}",resultado);
}
