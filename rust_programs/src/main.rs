/*
Pizza delivery basic program
*/
use std::io;

fn menu() {
    println!("Welcome to Mama Pacino!");
    println!("We hope to sate your appetite with our pizzas!");
}

fn main() {
    //pizza options
    let marg = 23.50;
    let pepp = 27.80;
    let cheese = 25.65;
    //percentage of price increase
    let small = 1.05;
    let medium = 1.12;
    let large = 1.25;

    menu();
    println!("\nType your name:");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("We couldn't read your name. Please, do type again!");
    let name = name.trim();

    println!("\nWhich pizza option would you like today?");
    println!("Type marg for Margherita; pepp for Pepperoni; cheese for Cheese.");
    let mut order = String::new();

    io::stdin()
        .read_line(&mut order)
        .expect("We couldn't read your order. Please, do type again!");
    let order = order.trim();

    println!("\nWhich pizza size would you like?");
    println!("Type: s for small; m for medium; l for large");
    let mut size = String::new();

    io::stdin()
        .read_line(&mut size)
        .expect("We couldn't read the size. Please, do type again!");
    let size = size.trim();

    if order == "marg" {
        if size == "s" {
            let cost_marg_s = marg * small;
            println!("\n{}, the final bill is {:.2} pounds", name, cost_marg_s);
        } else if size == "m" {
            let cost_marg_m = marg * medium;
            println!("\n{}, the final bill is {:.2} pounds", name, cost_marg_m);
        } else if size == "l" {
            let cost_marg_l = marg * large;
            println!("\n{}, the final bill is {:.2} pounds", name, cost_marg_l);
        } else {
            println!("We couldn't understand the demanded size. Please type again!")
        }

    } else if order == "pepp" {
        if size == "s" {
            let cost_pepp_s = pepp * small;
            println!("\n{}, the final bill is {:.2} pounds", name, cost_pepp_s);
        } else if size == "m" {
            let cost_pepp_m = pepp * medium;
            println!("\n{}, the final bill is {:.2} pounds", name, cost_pepp_m);
        } else if size == "l" {
            let cost_pepp_l = pepp * large;
            println!("\n{}, the final bill is {:.2} pounds", name, cost_pepp_l);
        } else {
            println!("We couldn't understand the demanded size. Please type again!")
        }

    } else if order == "cheese" {
        if size == "s" {
            let cost_cheese_s = cheese * small;
            println!("\n{}, the final bill is {:.2} pounds", name, cost_cheese_s);
        } else if size == "m" {
            let cost_cheese_m = cheese * medium;
            println!("\n{}, the final bill is {:.2} pounds", name, cost_cheese_m);
        } else if size == "l" {
            let cost_cheese_l = cheese * large;
            println!("\n{}, the final bill is {:.2} pounds", name, cost_cheese_l);
        } else {
            println!("We couldn't understand the demanded size. Please type again!")
        }

    } else {
        println!("\nWe didn't understand your order. Please type:");
        println!("marg for Margherita; pepp for Pepperoni; cheese for Cheese");
    }
}
