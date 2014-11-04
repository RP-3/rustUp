use std::io;
use std::rand;

fn compare(a:uint, b:uint) -> Ordering {
  if a<b {Less}
  else if a>b {Greater}
  else {Equal}
}

fn main(){
  println!("Guess which number I'm thinking of!");

  let secret_number = (rand::random::<uint>() % 100u) + 1u;

  loop{

    println!("Input your guess.");

    let input = io::stdin().read_line().ok().expect("Cannot read input line.");

    let input_num: Option<uint> = from_str(input.as_slice().trim());

    let num = match input_num {
      Some(num) => num,
      None      => {
        println!("Please input a Number.");
        continue;
      }
    };

    println!("You guessed {}", input);

    match compare(num, secret_number){
      Less => println!("Too small!"),
      Greater => println!("Too big!"),
      Equal => {
        println!("You win!");
        return;
      }
    }
  
  }

}
