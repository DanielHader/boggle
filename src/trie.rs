
// finds the shared prefix of strings s1 and s2
fn shared_prefix<'a>(s1: &'a str, s2: &str) -> &'a str {
    let min_len: usize = std::cmp::min(s1.len(), s2.len());
    let ind = std::iter::zip(s1[0..min_len].chars(),s2[0..min_len].chars()).position(|(c1,c2)| c1 != c2 );

    match ind {
	None => &s1[0..min_len],
	Some(ind) => &s1[0..ind],
    }
}

struct Node {
    text: String,
    children: Vec<Node>,
    terminal: bool,
}

pub struct Trie {
    root: Node,
}

impl Node {
    fn new(text: &str, terminal: bool) -> Node {
	Node { text: text.to_string(), children: Vec::<Node>::new(), terminal: terminal }
    }

    fn add_word(&mut self, word: &str) {
	if word.len() == 0 { self.terminal = true; return; }

	let mut found = false;
	for mut node in self.children.iter_mut() {
	    let prefix = shared_prefix(word, &node.text);
	    let word_rem = &word[prefix.len()..];
	    
	    if prefix.len() == node.text.len() {
		found = true;
		node.add_word(word_rem);
		break;
	    } else if prefix.len() > 0 {
		found = true;

		node.text = (&node.text[prefix.len()..]).to_string();
		let mut prefix_node = Node::new(prefix, prefix.len() == word.len());
		std::mem::swap(&mut prefix_node, &mut node);
		node.children.push(prefix_node);

		if prefix.len() < word.len() {
		    node.children.push(Node::new(word_rem, true));
		}
		
		break;
	    }
	}

	if !found {
	    let new_node = Node::new(word, true);
	    self.children.push(new_node);
	}
    }
    
    fn print(&self, prefix: &str, word: &str) {

	let new_word = format!("{}{}", word, self.text);
	
	match self.terminal {
	    true => println!("\"{}\" (\"{}\")", self.text, new_word),
	    false => println!("\"{}\"", self.text),
	};
	
	let mut it = self.children.iter().peekable();
	while let Some(node) = it.next() {
	    if it.peek().is_none() {
		print!("{}└", prefix);
		let new_prefix = format!("{} ", prefix);
		node.print(&new_prefix, &new_word);
	    } else {
		print!("{}├", prefix);
		let new_prefix = format!("{}│", prefix);
		node.print(&new_prefix, &new_word);
	    }
	}
    }
}

impl Trie {
    /// Creates a trie from a list of words
    pub fn from_words(words: &Vec<&str>) -> Trie {
	let mut trie = Trie { root: Node::new("", false) };
	for word in words {	trie.add_word(word); }
	return trie;
    }

    pub fn print(&self) {

	let prefix = "";
	let word = "";

	self.root.print(prefix, word);
    }

    /// Adds a single word to the trie.
    pub fn add_word(&mut self, word: &str) {
	self.root.add_word(word);
    }

    pub fn has_prefix(word: &str) {
	
    }
}
