use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod trie;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {

    if let Ok(lines) = read_lines("/usr/share/dict/words") {

	for line in lines.take(10).flatten() {
	    println!("{}", line);
	}
    }
    
    let mut words = Vec::<&str>::new();
    words.push("romane");
    words.push("romanus");
    words.push("romulus");
    words.push("rubens");
    words.push("ruber");
    words.push("rubicon");
    words.push("rubicundus");
    
    let trie = trie::Trie::from_words(&words);

    trie.print();
}
