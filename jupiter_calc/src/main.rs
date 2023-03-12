use std::io;

fn main() {
    println!("Enter the weight on earth (kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    //remove white spaces with trim and unwarp to handle fails
    let weight: f32 = input.trim().parse().unwrap();
    dbg!(weight);

    println!("Weight on earth (kg): {}", weight);
    let jupiter_weight = calculate_weight_on_jupiter(weight);
    println!("Weight on Jupiter : {}kg",jupiter_weight);
}
  
fn calculate_weight_on_jupiter(weight: f32) -> f32 {
    (weight/9.81)*24.79
}
