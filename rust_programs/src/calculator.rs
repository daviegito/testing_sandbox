use std::io;

fn menu() {
    println!("Bem-vindo à calculadora tosca");
}

fn main() {
    menu();
    
    println!("\nDigite um valor para x, primeiro valor:");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("não consegui ler x");
    println!("Seu x escolhido foi: {x}");
    
    println!("Digite um valor para y, segundo valor:");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("não consegui ler y");
    println!("Seu y escolhido foi {y}");
    
    let x: i32 = x.trim().parse().expect("input inválido");
    let y: i32 = y.trim().parse().expect("input inválido");
    
    println!("\nVocê precisa calcular alguma coisa? Responda com sim ou nao");
    let mut escolha1 = String::new();
    io::stdin().read_line(&mut escolha1).expect("não consegui ler a escolha");
    let escolha1 = escolha1.trim();
    if escolha1 == "sim" {
        println!("Você optou por {escolha1}!");
        println!("\nVamos aos cálculos então!");
    } else {
        println!("Sem cálculos por agora, né? Tranquilo. Até mais!");
    }
    
    while escolha1 == "sim" {
        println!("Digite uma entre as operações que você deseja: ");
        println!("soma, multiplicacao, divisao, subtracao\n");
        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("não consegui ler a escolha");
        let escolha = escolha.trim();
        println!("\nVocê digitou: {}", escolha);
        
        match escolha {
                "soma" => {
                    let soma = x + y;
                    println!("Soma: {x} + {y} = {}", soma);
                
                }
                "multiplicacao" => {
                    let multiplicacao = x * y;
                    println!("Multiplicacão: {x} * {y} = {}", multiplicacao);
                    
                }
                "divisao" => {
                    let divisao = x / y;
                    println!("Divisão: {x} / {y} = {}", divisao);
                    println!("Módulo da divisão: {x} % {y} = {}", x % y);
                    
                }
                "subtracao" => {
                    let subtracao = x - y;
                    println!("Subtração: {x} - {y} = {}", subtracao);
                    
                } _ => {
                    println!("Opção inválida. Encerrando o programa!");
                    break;
                }
        } 
    };
}