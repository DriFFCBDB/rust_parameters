//Crie uma função que receba uma &str como parâmetro e retorne o número de caracteres na &str.

fn strin_param (texto : &str) -> u32 {
    
    let texto = "Usurpradora";

    let tamanho :u32 = texto.len() as u32;

    tamanho * if tamanho > 4 { 1 } else { 0 }

}

fn main() {
 
    let texto = "Texto Pequeno";

    let resultado = strin_param(texto);

    println!("o Tamanho é : {}",resultado)

}
