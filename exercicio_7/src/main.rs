//Crie uma função que receba dois números inteiros como parâmetros e retorne o maior deles.
fn maior_deles(a :u32 , b: u32) -> u32{

    if a > b {
        a
    }else{
        b
    }
}

fn main() {

   let a = 80;
   let b = 20;

   let resultado = maior_deles(a,b);

   println!("O maior é: {} ",resultado)
}
