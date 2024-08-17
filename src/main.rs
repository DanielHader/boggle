use std::env;
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

    let args: Vec<String> = env::args().collect();
    let mut dict_filepath = env::args().nth(1).unwrap_or("/usr/share/dict/words".to_string());

    let mut words = Vec::<String>::new();
    if let Ok(lines) = read_lines(dict_filepath) {
	words = lines.filter_map(|r| r.ok()).filter(|x| x.chars().all(|c| c.is_ascii_lowercase())).take(20).collect();
    }
    let word_refs: Vec<&str> = words.iter().map(|s| &**s).collect();
    
    let trie = trie::Trie::from_words(&word_refs);
    trie.print();
}
