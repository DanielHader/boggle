mod trie;

fn main() {

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
