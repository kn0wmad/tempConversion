use std::io;
//use std::cmp;

fn main() {
    println!("Temperature Conversion Tool");

    println!("Select Fahrenheit (F) or Celsius (C):");

    let mut input0 = String::new();

    io::stdin().read_line(&mut input0).expect("Please enter F or C:");

    let input0 = input0.trim();

    match input0 {
        "F" | "f" => {
            println!("You've selected Fahrenheit");
            println!("Please enter temperature");

                let mut input1 = String::new();

                io::stdin().read_line(&mut input1).expect("Please enter a number");

                let mut temp = input1.trim().parse::<f32>().unwrap();

                temp = temp - 32.0 * 1.8;

                println!("The temperature in Celsius is: {}", temp);
        }

        "C" | "c" => {
            println!("You've selected Celsius");
            println!("Please enter temperature");

                let mut input1 = String::new();

                io::stdin().read_line(&mut input1).expect("Please enter a number");

                let mut temp = input1.trim().parse::<f32>().unwrap();

                temp = temp * 1.8 + 32.0;

                println!("The temperature in Fahrenheit is: {}", temp);
        }
        
        _ => println!("Please enter F or C", ),
    }
}
