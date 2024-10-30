mod tail;
use tail::tail_string_delim;

mod read_delim;

fn main() {
    println!("{}", tail_string_delim("./tests/files/cages birds.txt".to_string(), 1, "i".to_string()));
}