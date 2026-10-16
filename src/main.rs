use std::io;
use rand::RngExt;
use std::cmp::Ordering;


fn main() {
    println!("猜数游戏! !");
    let secret_number = rand::rng().random_range(1..101); // i32 u32 i64
    
    loop {
        println!("猜测一个数");

        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("无法读取行");
        
        // shadow
        let guess: u32 = match guess.trim().parse(){
          Ok(num) => num,
          Err(_) => continue,
        };
        
        println!("你猜测的数是: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小啦！"), // arm
            Ordering::Greater => println!("太大啦!"),
            Ordering::Equal => {    
                println!("猜对了，你赢了！");
                break;
            },
        }
    }
}