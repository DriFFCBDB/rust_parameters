// Crie uma função que receba um número inteiro como parâmetro e retorne o seu quadrado.

fn fatorial(numero_inteiro:i32) -> i32{

    numero_inteiro * numero_inteiro
}

fn main() {
    let resultado:i32 = fatorial(9);
    println!("O fatorial é: {}", resultado);
}
