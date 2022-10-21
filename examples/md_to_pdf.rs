/*
use crowbook::Book;
// Reads configuration file "foo.book" and render all formats according to this
// configuration file
fn main() {
    /*
    Book::new().load_file("foo.book").unwrap().render_all().unwrap();
    */
    let mut book = Book::new();
    let _ = book.load_markdown_file("./examples/template.md");
    /*book.set_options(&[("author", "Joan Doe"),
                       ("title", "An untitled book"),
                       ("lang", "en")]);*/
         // Add a chapter to the book
    //book.add_chapter_from_source(Number::Default, "# The beginning#\nBla, bla, bla".as_bytes()).unwrap();
         // Render the book as html to stdout
    let mut pdf: Vec<u8> = Vec::new();
    //book.render_format_to("pdf", &mut pdf).unwrap();
    //book.render_format_to("pdf", &mut std::io::stdout()).unwrap();
    book.render_format_to_file("pdf", "./target/book.pdf").unwrap();
}*/
fn main() {

}