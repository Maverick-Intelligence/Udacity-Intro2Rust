struct Book<'author> {
    title: String,
    author: &'author str,
}

fn print_book_details(book: &Book) {
    println!("Book Name: {}", book.title);
    println!("Author Name: {}", book.author);
}

fn main() {
    let title: String = String::from("Artificial Intelligence A Modern Approach");
    let author: &str = "Stuart Russel & Peter Norvig";
    let book: Book = Book { title, author };
    print_book_details(&book);
}
