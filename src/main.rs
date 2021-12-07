fn main() {
    std::println!("Enter your current grade: ");
    let mut string_1 = String::new();
    std::io::stdin()
        .read_line(&mut string_1)
        .expect("Error reading");
    let current = string_1.trim().parse::<f32>().expect("NaN");
    std::println!("Enter your wanted grade: ");
    let mut string_2 = String::new();
    std::io::stdin()
        .read_line(&mut string_2)
        .expect("Error reading");
    let goal = string_2.trim().parse::<f32>().expect("NaN");
    std::println!("Enter final weight percentage: ");
    let mut string_3 = String::new();
    std::io::stdin()
        .read_line(&mut string_3)
        .expect("Error reading");
    let weight = string_3.trim().parse::<f32>().expect("NaN") / 100.0;
    let needed = (goal - (current * (1.0 - weight))) / weight;
    std::println!("You need a {}%", needed);
}
