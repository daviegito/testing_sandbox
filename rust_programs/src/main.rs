/*
Pizza delivery basic program
*/
use std::io;

fn menu() {
    println!("Welcome to Mama Pacino!");
    println!("We hope to sate your appetite with our pizzas!");
}

fn cost_marg_s() -> f32 {
    println!("\nThe final bill is 24.67 pounds");
    return 24.67;
}

fn cost_marg_m() -> f32 {
    println!("\nThe final bill is 26.32 pounds");
    return 26.32;
}

fn cost_marg_l() -> f32 {
    println!("\nThe final bill is 29.37 pounds");
    return 29.37;
}

fn cost_pepp_s() -> f32 {
    println!("\nThe final bill is 29.19 pounds");
    return 29.19;
}

fn cost_pepp_m() -> f32 {
    println!("\nThe final bill is 31.13 pounds");
    return 31.13;
}

fn cost_pepp_l() -> f32 {
    println!("\nThe final bill is 34.75 pounds");
    return 34.75;
}

fn cost_cheese_s() -> f32 {
    println!("\nThe final bill is 26.93 pounds");
    return 26.93;
}

fn cost_cheese_m() -> f32 {
    println!("\nThe final bill is 28.72 pounds");
    return 28.72;
}

fn cost_cheese_l() -> f32 {
    println!("\nThe final bill is 32.06 pounds");
    return 32.06;
}

fn main() {
    menu();

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

    full_order(order, size);
}

pub fn full_order(order: &str, size: &str) {
    if order == "marg" {
        if size == "s" {
            cost_marg_s();
        } else if size == "m" {
            cost_marg_m();
        } else if size == "l" {
            cost_marg_l();
        } else {
            println!("We couldn't understand the demanded size. Please type again!")
        }

    } else if order == "pepp" {
        if size == "s" {
            cost_pepp_s();
        } else if size == "m" {
            cost_pepp_m();
        } else if size == "l" {
            cost_pepp_l();
        } else {
            println!("We couldn't understand the demanded size. Please type again!")
        }

    } else if order == "cheese" {
        if size == "s" {
            cost_cheese_s();
        } else if size == "m" {
            cost_cheese_m();
        } else if size == "l" {
            cost_cheese_l();
        } else {
            println!("We couldn't understand the demanded size. Please type again!")
        }

    } else {
        println!("\nWe didn't understand your order. Please type:");
        println!("marg for Margherita; pepp for Pepperoni; cheese for Cheese");
    }
}

#[cfg(test)]
mod tests  {
    use super::*;

    #[test]
    fn test_small_marg() {
        let expected = 24.67;
        let result:f32 = cost_marg_s();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_med_pepp() {
        let random_value = 23.45;
        let result:f32 = cost_pepp_m();
        assert_ne!(random_value, result);
    }

    #[test]
    #[should_panic]
    fn test_panic_wrong_value() {
        let random_value = 98.34;
        let result:f32 = cost_cheese_l();
        assert_eq!(random_value, result);
    }
}