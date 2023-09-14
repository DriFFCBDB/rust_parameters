//Crie uma função que receba uma string como parâmetro e retorne verdadeiro se a string for um palíndromo (ou seja, pode ser lida da mesma forma da esquerda para a direita e vice-versa) e falso caso contrário.

fn is_palindromo(palavra :&str) -> bool{

    let palavra_reversa :String = palavra.chars().rev().collect();

    if palavra == palavra_reversa{
        println!("É polindromo.");
        true
    }else{
        println!("Não é.");
        false
    }
}

fn main() {
    
    let palavra = "ana";
    let resultado = is_palindromo(palavra);
    println!("{}",resultado);
}
