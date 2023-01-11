// Constante global a mesma coisa que a normal
// Variável global: static
// Uso de variáveis globais mutáveis é inseguro, o rust nao permite. Precisa colocar unsafe{} na hora de usar
static mut VAR_GLOBAL:u8 = 1;

fn sintaxe() {
  //AULA 1 = Tipos e Variáveis
  //AULA 2 = Constantes
  //AULA 3 = Recursos específicos:
  // 1- Redeclarar variável
  
  // valor da variável não pode ser modificado, só com let mut
  // tipo é opcional, o padrão é :i32 (inteiro 32bytes)
  
  // u = unsigned sempre positivo
  let variavelu:u8 = 1;
  
  // i = inteiro com bit de neg ou positivo, 
  let variaveli:i8 = 1;
  
  // variavel decimal
  let decimal:f32 = 2.5;
  
  //variavel booleana
  let booleana:bool = true; //ou false

  //variavel char
  let letra:char = 'C';

  //Sabe que é inseguro mas quer fazer mesmoa assim:
  unsafe{
    println!("Global: {}", VAR_GLOBAL);
  }
  println!("Variável U: {}", variavelu);
  println!("Variável I: {}", variaveli);
  println!("Variável F: {}", decimal);
  println!("Variável B: {}", booleana);
  println!("Variável C: {}", letra);

  
  //Variável armazena o valor na memória, constante não
  const PI:f32 = 3.14;
  
  println!("PI: {}", PI);

  
}

fn func(a:i32,b:i32) -> i32 {
  //Aula 4- Funções
  println!("{} + {} = {}", a,b,a+b);
  // nesse caso nao precisa do ; vale como return a+b;
  a+b
  
}

fn estruturas_de_controle(){
  // Aula 5- IF, ELSE, Match
  //IF normal com else if, &&, ||...
  let idade: u8 = 18;
  if idade >= 18{
    println!("Maior de idade");
  } else {
    println!("Menor de idade");
  }
  // ou...
  let eh_maior = idade >= 18;
  if eh_maior{
    println!("Maior de idade");
  }
  // ou...
  let condicao = if idade > 17 {"maior"} else {"menor"};
  println!("É {} de idade",condicao);
  
  // Switch case = Match do python
  let numero = "3";//muda aqui e é mudado la
  let ranking = match numero{
    "1" => "primeiro",
    "2" => "segundo",
    "3" => "terceiro",
    _ => "Desconhecido"
  };
  println!("O lugar {} é o {}",numero,ranking);

  //Aula 6- For, While, Loop
  //LOOPS
  let mut contador:u8 = 0;
  //While
  while contador < 10{
    print!("{},",contador);
    contador += 1;
  }
  contador = 0;
  //loop = infinito, tem que ter um break; para sair dele
  // tem continue; tbm
  loop{
     contador += 1;
    if contador == 10 {
      println!("Já é 10");
      break;
    }
  }
  //FOR = igual ao python
  //1..11 (deixa 1 tira 11)
  //1..=10 (deixa 1 e 10)
  for i in 1..11{
    print!("{},",i);
  }
  

}

fn ownership(){
  //Aula 7- Ownership
  // A string não possui esse ("") valor, mas possui um "ponteiro" para o espaço de memória que possui esse valor
  // Aloca a memória na RIP e passa o ponteiro na Stack, pq o tamanho da string pode ser muito grande, se armazenar tudo na Stack, ocorre o StackOverflow
  // String dinâmica = RIP
  // String estática = RIP ou Stacl (compilador decide)
  // Cada valor só tem 1 dono (ownership), se usa a string em outra função (por parâmetro), a posse do valor move para a outra , enquanto a outra fica inválida
  
  // Uma solução (gambiarra) para isso é retornar o valor para a função inicial
  let uma_string = String::from("Teste1");  // Isso é uma string estática tratada como string dinâmica
  let outra_string = rouba(uma_string);
  println!("{}",outra_string);


  //Aula 8- Referências e Borrowing
  //Outra solução é emprestar a posse do valor, ou seja, ao inves do dono mudar, o dono empresta o valor, passando ele por referência
  let terceira_string = String::from("Teste2");
  rouba_outra(&terceira_string)


  

}
//usa string mas retorna -> String
fn rouba(string: String) -> String{
  //sempre que um valor sai do escopo, esse valor da rip é liberado
  println!("{}",string);
  
  string
}

//usa apenas a referencia da string (pega emprestado)
//se precisar mudar, a referencia deve ser mutavel, ou seja, &mut String, mas a variável tbm tem que ter sido criada mutável
fn rouba_outra(string: &String){
  println!("{}",string);
}

fn pattern_matching() {
//Aula 9- Pattern Matching
//Você tem um valor e quer combinar com outros diferentes
    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Um pouquinho",
            4..=10 => "Um bocado",
            _ if x % 2 == 0 => "Uma boa quantidade",
            _ => "Muito"
        });
    }
}

fn erros() {
  //Aula 10- "Tratamento" de erros
    match resultado() {
        Ok(s) => println!("String de sucesso = {}", s),
        Err(numero) => println!("Codigo de erro = {}", numero)
    };
}

fn resultado() -> Result<String, u8>
{
  //Código de erro
    Err(42)
}
fn main(){
  sintaxe();
  func(2,2);
  estruturas_de_controle();
  println!(" ");
  ownership();
  pattern_matching();
  erros();
}

// Se precisar usar cógigos externos / dependências:
// -> Cargo: gerenciador de pacotes <- 

// cargo --help
// cargo run (ao inves de compilar quem nem C)