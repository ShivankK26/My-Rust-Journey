fn main() {
    // One instance of the Book Struct
    let book = Book {
        title: String::from("The way of Zen!"),
        author: String::from("John Deans"),
        publication_year: 1957,
    };

    println!(
        "The book {} is written by {} in {}",
        book.title, book.author, book.publication_year
    );

    // Defining a mutable instance of the book variable
    let mut book = Book {
        title: String::from("The way of Zen!"),
        author: String::from("John Deans"),
        publication_year: 1957,
    };

    book.publication_year = 1980;

    println!(
        "The book {} is written by {} in {}",
        book.title, book.author, book.publication_year
    );

    // Using the function which we created
    let book_data = get_book_data(book);
    for data in book_data {
        println!("{data}");
    }

    // Using the create_book function here
    let my_book = create_book("The path of zen".to_string(), "Simon".to_string(), 2023);

    println!("My book is {:?}", my_book); // Using the #[derive(Debug)] which we created below


    let tuple_book = Tuple_Book("Some book".to_string(), "Simon".to_string(), 2023);

    let title = tuple_book.0;
    let author = tuple_book.1;
    let publication_year = tuple_book.2;

    let unit_book = Unit_Book;

    // Importing the struct and impl
    let my_rectangle = Rectangle {
        width: 10.0,
        height: 5.0,
    }
    let area = my_rectangle.area();
    println!("The area of the rectangle is: {}", area);
}

// Below, we're using the Debug mode, which when used returns the variables in the struct when we run the code
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    publication_year: u32,
}


// Creating a tuple
struct Tuple_Book(String, String, u32);


// Creating a Unit struct
struct Unit_Book;


// Creating a function for getting the book data
fn get_book_data(book: Book) -> [String; 3] {
    let title = book.title;
    let author = book.author;
    let publication_year = book.publication_year;

    let data: [String; 3] = [title, author, publication_year.to_string()];

    data
}

// Creating a book function
fn create_book(title: String, author: String, publication_year: u32) -> Book {
    let book = Book {
        title: title,
        author: author,
        publication_year: publication_year,
    };

    book
}


struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle { // implementing a method called Rectangle
    fn area(&self) -> f64 {
        self.width * self.height
    }
} 