use std::io;

fn main() {
    println!("Enter your weight (kg)");

    let mut input =  String::new();

    io::stdin().read_line( &mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    println!("Input: {}", input);

    let _mars_weight = weight_calculations(100.0);
    // mars_weight = mars_weight * 1000.0;

    println!("Weight on mars : {}kg!", weight_calculations(100.0));
}

fn weight_calculations(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
