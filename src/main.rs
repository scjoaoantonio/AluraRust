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

fn array(){
  // Pode usar também -> usize e isize <- (quando é o compilador que decide o tipo e tamanho de bits de acordo com o computador que está fazendo a compilação)
  
  //let notas = [10f32,8f32,9.5,6.0];
  //ou para especificar o tipo e a quantidade:
  let notas: [f32; 4] = [10.0,8.0,9.5,6.0];
  for nota in notas{
    println!("Nota {}",nota);
  }
  //ou usando o indice
  for indice in 0..notas.len(){
    println!("Posição {} => Nota {}",indice+1,notas[indice]);
  }
  // array com o mesmo múmero:
  let repete: [f32; 4] = [6.0;4];
  for num in repete{
    println!("Numero repetido: {}",num);
  }
}

fn matriz(){
  //para criar a matriz:
  let matriz: [[f32;3];3] = [
    [0.0,1.0,2.0],
    [3.0,4.0,5.0],
    [6.0,7.0,8.0]
  ];
  // para percorrer a matriz:
  for linha in matriz{
    for item in linha{
      println!("Num {}",item);
    }
  }
}

//pode criar variáveis e valores com esse tipo, como se fosse um tipo de variável (inteiro, float etc)
//para tirar os warnings tem que usar todos os tipos (seg,ter,qua,qui..)
enum DiaDaSemana{
  Domingo,
  Segunda,
  Terça,
  Quarta,
  Quinta,
  Sexta,
  Sabado
}

fn fds(dia_da_semana: DiaDaSemana) -> bool{
  match dia_da_semana{
    DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
    _ => false
  }
}


fn enumeracoes(){
  println!("é fim de semana? {}", fds(DiaDaSemana::Domingo));
}

// Vai ler um arquivo ou conteudo, se tiver vazio avisa, senao, manda o conteudo
fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));

    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo nao existe")
    };

    println!("{:?}", conteudo_arquivo);

    if let Some(valor) = conteudo_arquivo {
        println!("Agora, tenho certeza de ha o valor {}", valor);
    }
}

fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    Some(String::from("Algum conteudo"))
}

fn vectors(){
  //ARRAY: vetores com tamanhos prá definidos
  //VECTORS: vetores com tamanhos dinamicos
  let mut notas: Vec<f32> = Vec::new();
  notas.push(10.0);
  notas.push(8.8);
  notas.push(6.5);
  //OUU
  let notas2: Vec<f32> = vec![10.0,8.8,6.5];

  println!("{:?}",notas);
  println!("{:?}",notas2);

  //ou focar na capacidade do vec
  let notas3: Vec<f32> = Vec::with_capacity(4);
  notas.push(10.0);
  notas.push(8.8);
  notas.push(6.5);
  //aqui tem capacidade para 4 elementos mas só utiliza 3, pq é pode ter menos mas nao pode ter mais que a capacidade total
  println!("Capacidade = {}", notas3.capacity());
  
  //para acessar um valor específico
  println!("Nota 1 = {}", notas[0]);

  //acessar um valor se tiver ou nao
  println!("Nota 6 = {}", match notas.get(7){
    Some(n) => *n,
    None => 0.0
  });

  //.pop() -> Pega o ultimo valor, retorna e exclui ele
  //if let some = se tiver algum valor..
  if let Some(nota) = notas.pop(){
    println!("Ultimo valor = {}",nota);
    println!("{:?}",notas);
  }

  //percorrendo um vec
  for nota in &notas{
    println!("Nota = {}", nota);
  }
  println!("{:?}",notas);
}

//tipo um classe
struct Conta {
  titular: Titular,
  saldo: f64
}

//método tipo POO, tipo funções
//self = referencia à struct original (&), se for mudar tem que avisar com &mut
impl Conta{
  fn sacar(&mut self,valor: f64){
    self.saldo -= valor;
  }
}

struct Titular{
  nome: String,
  sobrenome: String
}

fn conta_corrente(){
  let titular = Titular{nome: String::from("Joao"),sobrenome: String::from("Antonio")};
  let mut conta: Conta = Conta{
    titular,
    saldo: 100.0
  };

  conta.sacar(50.0);
  
  println!("Dados da conta: Titular = {} {}, Saldo = {}", conta.titular.nome, conta.titular.sobrenome, conta.saldo);
}

fn main(){
  println!("---------PARTE 1------------");
  sintaxe();
  func(2,2);
  estruturas_de_controle();
  println!(" ");
  ownership();
  pattern_matching();
  erros();
  println!("---------PARTE 2------------");
  array();
  matriz();
  enumeracoes();
  conteudo_opcional();
  vectors();
  conta_corrente();
}

// Se precisar usar cógigos externos / dependências:
// -> Cargo: gerenciador de pacotes <- 

// cargo --help
// cargo run (ao inves de compilar quem nem C)