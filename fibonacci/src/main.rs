use std::io;
fn main() {
   let mut a = 0;
   let mut x = 0;
   let mut y = 1;
   let mut number = String::new();
   println!("Please input the number of Fibonacci's you want.");
   io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

   let number: u32 = number.trim().parse().expect("Please type a number!");
   for index in 1..number+1 {
       println!("Fibonacci postion: {}   is equal to : {}", index, a);
       x = y;
       y = a;
       a = x + y;
   }
}
