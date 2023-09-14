//Crie uma função que receba um número inteiro como parâmetro e retorne verdadeiro se o número for par e falso caso contrário.


fn receber_inteiro(numero_inteiro :u32) -> bool {

    //divisivel por 2 

    if numero_inteiro % 2 == 0 {
        println!("É par.");
         true
        }else{
            println!("Não é par.") ;

            false
        }
}


fn main() {
    let numero_inteiro = 6;
    let resultado = receber_inteiro(numero_inteiro);

    println!("{}",resultado);
}
