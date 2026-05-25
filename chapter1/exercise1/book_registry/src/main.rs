struct Book<'author> {
    title: String,
    author: &'author str,
}

fn print_book_details(book: &Book) {
    println!("Author: {}", book.author);
    println!("Title: {}", book.title);
}

fn main() {
    let title_book_1 = String::from("Artificial Intelligence A Modern Approach");
    let author_book_1 = "Peter Norvig & Stuart Russel";

    let book_1 = Book {
        title: title_book_1,
        author: author_book_1,
    };
    print_book_details(&book_1);
}
