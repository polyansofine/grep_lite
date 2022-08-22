fn main() {
    let search_text = "picture";
    let quote = "\
        Every face, every shop, bedroom window, public-house, and 
        dark square is a picture feverishly turned--in search of what? 
        It is the same with books.
        What do we seek through millions of pages?
    ";
    // let mut line_num = 1;

    for (i,line) in quote.lines().enumerate() {
        if line.contains(search_text) {
            println!("{}:{}",i+1,line);
        }
        // line_num +=1;
    }
}
