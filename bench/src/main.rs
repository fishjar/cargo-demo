use hello_word;
use hi;

fn main() {
    println!("bench start");
    hello_word::say();
    hi::say();

    #[cfg(feature = "say-if")]
    println!("say if!!");

    #[cfg(feature = "say-again")]
    again::say();

    println!("bench end");
}
