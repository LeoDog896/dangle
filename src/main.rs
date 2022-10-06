use memorable_wordlist::WORDS;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let mut rng = thread_rng();
    println!("{:?}", WORDS.choose(&mut rng));
}
