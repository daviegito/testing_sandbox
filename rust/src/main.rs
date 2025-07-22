/*
Programa básico de pedido de pizza
*/
use std::io;
use std::ops::Mul;

fn menu() {
    println!("Seja bem-vindo/a à pizzaria Mama Pacino!");
    println!("Esperamos saciar a sua fome com as nossas deliciosas pizzas!");
}

enum Pedido {
    Cala(f64), //calabresa
    Port(f64), //portuguesa
    Palm(f64), //palmito
}

enum Tamanho {
    P(f64),
    M(f64),
    G(f64),
}

impl Mul<Tamanho> for Pedido {
//Implementação do traço mul
    type Output = f64;

    fn mul(self, tamanho: Tamanho) -> f64 {
        let base = match self {
            Pedido::Palm(v) => v,
            Pedido::Cala(v) => v,
            Pedido::Port(v) => v,
};

        let fator = match tamanho {
            Tamanho::P(f) => f,
            Tamanho::M(f) => f,
            Tamanho::G(f) => f,
};

        base * fator
}
}

fn main() {
    //sabores de pizza
    let cala = Pedido::Cala(23.50);
    let port = Pedido::Port(27.80);
    let palm = Pedido::Palm(25.65);
    //percentagem de aumento de preço dado o tamanho
    let peq = Tamanho::P(1.05);
    let med = Tamanho::M(1.12);
    let gra = Tamanho::G(1.25);

    menu();
    //primeiramente, o cliente irá se apresentar
    println!("\nDigite o seu nome:");
    let mut nome = String::new();

    io::stdin()
        .read_line(&mut nome)
        .expect("Não conseguimos ler o seu nome. Favor, tentar digitar novamente");
    let nome = nome.trim();

    //agora, digitar qual pizza
    println!("\nQual a pizza que você quer hoje?");
    println!("Digite cala para calabresa; port para portuguesa; palm para palmito.");
    let mut pedido = String::new();

    //ler o pedido
    io::stdin()
        .read_line(&mut pedido)
        .expect("Não conseguimos ler o seu pedido. Favor, tentar digitar novamente");
    let pedido = pedido.trim();

    //por fim, o tamanho da pizza
    println!("\nQual o tamanho da pizza que você quer?");
    println!("Digite: p para pequeno; m para médio; g para grande");
    let mut tamanho = String::new();

    //ler o tamanho
    io::stdin()
        .read_line(&mut tamanho)
        .expect("Não conseguimos ler o tamanho. Favor, tentar digitar novamente");
    let tamanho = tamanho.trim();

    if pedido == "cala" {
        if tamanho == "p" {
            println!("\n{}, o preço foi de {:.2} reais", nome, cala * peq);
        } else if tamanho == "m" {
            println!("\n{}, o preço foi de {:.2} reais", nome, cala * med);
        } else if tamanho == "g" {
            println!("\n{}, o preço foi de {:.2} reais", nome, cala * gra);
        } else {
            println!("Não conseguimos entender o tamanho. Favor, digitar novamente!")
        }

    } else if pedido == "port" {
        if tamanho == "p" {
            println!("\n{}, o preço foi de {:.2} reais", nome, port * peq);
        } else if tamanho == "m" {
            println!("\n{}, o preço foi de {:.2} reais", nome, port * med);
        } else if tamanho == "g" {
            println!("\n{}, o preço foi de {:.2} reais", nome, port * gra);
        } else {
            println!("Não conseguimos entender o tamanho. Favor, digitar novamente!")
        }

    } else if pedido == "palm" {
        if tamanho == "p" {
            println!("\n{}, o preço foi de {:.2} reais", nome, palm * peq);
        } else if tamanho == "m" {
            println!("\n{}, o preço foi de {:.2} reais", nome, palm * med);
        } else if tamanho == "g" {
            println!("\n{}, o preço foi de {:.2} reais", nome, palm * gra);
        } else {
            println!("Não conseguimos entender o tamanho. Favor, digitar novamente!")
        }

    } else {
        println!("\nNós não conseguimos entender o seu pedido. Favor digitar:");
        println!("cala para calabresa; port para portuguesa; palm para palmito");
    }
}


