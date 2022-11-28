use md5::{Digest, Md5};

fn main() {
    let input = "iwrupvqb";
    let x = 346386.to_string();

    let mut hasher = Md5::new();
    hasher.update(input);
    hasher.update(x.as_bytes());

    let result = hasher.finalize().to_vec();
    println!("{result:?}");
}
