use fltk::{
    app,
    button::Button,
    image::SvgImage,
    group::Flex,
    prelude::*,
    window::Window
};

mod boggle;

//use std::env;
//use std::fs::File;
//use std::io::{self, BufRead};
//use std::path::Path;

//mod trie;

//fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
//where P: AsRef<Path> {
//    let file = File::open(filename)?;
//    Ok(io::BufReader::new(file).lines())
//}

fn main() {

    let app = app::App::default();
    
    let letters: Vec<&str> = vec!["a", "b", "c", "d",
				  "e", "f", "g", "h",
				  "i", "j", "k", "l",
				  "m", "n", "o", "p"];
    
    let _ = boggle::BoggleBoard::<4,4>::new(&letters);

    let mut window = Window::default().with_size(120, 120).with_label("window");
    let mut flex = Flex::default_fill().column();
    flex.set_margins(10, 10, 10, 10);
    flex.set_pad(10);

    let mut button = Button::default().with_label("button");
    let w = button.width();
    let h = button.height();
    
    let mut image = SvgImage::load("img/cube.svg").unwrap();
    image.scale(w, h, true, true);
    
    button.set_image(Some(image.clone()));
    button.set_deimage(Some(image));
    button.set_compact(true);

    flex.end();

    window.make_resizable(true);
    
    window.end();
    window.show();
    
    app.run().unwrap();
    
    /*
    let args: Vec<String> = env::args().collect();
    let mut dict_filepath = env::args().nth(1).unwrap_or("/usr/share/dict/words".to_string());

    let mut words = Vec::<String>::new();
    if let Ok(lines) = read_lines(dict_filepath) {
	words = lines.filter_map(|r| r.ok()).filter(|x| x.chars().all(|c| c.is_ascii_lowercase())).take(20).collect();
    }
    let word_refs: Vec<&str> = words.iter().map(|s| &**s).collect();
    
    let trie = trie::Trie::from_words(&word_refs);
     */

    
}
