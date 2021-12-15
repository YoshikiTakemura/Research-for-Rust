fn main() {
    println!("Processing...");
    for i in 2..100000 {
        for j in 2..i {
            if i % j == 0{
                break;
            }
        }
    }
    println!("Processing completed");
}