use std::io;

fn main() {
    println!("Enter Earth weight (kg): ");
    let mut input = String::new(); // Create an empty mutable string
    io::stdin().read_line(&mut input).unwrap(); // Pass a reference inside - value is 'borrowed' without copying the pointer.
    // unwrap() fails the program if the result is fail.

    let mut reference_to_string = &mut input; // A botrrowed value. Will not deallocate the value, when the variable goes out of scope.
    println!("Input: {}", reference_to_string); 
    println!("Input: {}", input);

    let weight: f32 = input.trim().parse().unwrap();
    // dbg!(weight);

    let mut mars_weight = calculate_weiht_on_mars(weight);
    mars_weight = mars_weight * 1000.0;
    println!("Initial: {}kg, Mars weight: {}g", weight, mars_weight);
    
    borrow_string(&input);
    println!("Input after ownership change {}", input);
    own_string(input);
    // println!("Input after ownership change {}", input); // Will fail, as ownership was taken.
}

fn calculate_weiht_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn borrow_string(s: &String) {
    println!("Borrowed: {}", s);
}

fn own_string(s: String) {
    println!("Ownership taken by the fn: {}", s);
}