use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
        println!("Enter No. Guess");

    loop {
        
        println!("Input Guess");

        let sec_num = rand::thread_rng().gen_range(0..=10);

        let mut guess= String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess
            .trim().parse()
            .expect("Please enter number");

        println!("Your guess is: {guess}");

        match guess.cmp(&sec_num) {
            Ordering::Less => println!("Lesser than: {sec_num}"),
            Ordering::Greater => println!("Greater than: {sec_num}"),
            Ordering::Equal => {
                println!("Equal to: {sec_num}");
                break;
            }
        }
    }
}
