// The main library module
pub mod library {
    // Sub-module for books
    pub mod books {
        // Struct and its fields must be marked as `pub` to be accessible outside this module
        pub struct Book {
            pub title: String,
            pub year: u32,
        }
    }

    // Sub-module for writers
    pub mod writers {
        // We need to import Book from the sibling module `books`
        use super::books::Book;

        pub struct Writer {
            pub first_name: String,
            pub last_name: String,
            pub books: Vec<Book>,
        }
    }
}

use library::writers::Writer;

// The function is outside the modules, as requested
pub fn order_books(writer: &mut Writer) {
    // We use `sort_by` to provide a custom comparison function.
    // By converting both titles to lowercase before comparing, we make the sort case-insensitive.
    writer.books.sort_by(|a, b| {
        a.title.to_lowercase().cmp(&b.title.to_lowercase())
    });
}

/*
pub mod library {
    pub mod books {
        pub struct Book {
            pub title: String,
            pub year: u32,
        }
    }

    pub mod writers {
        use super::books::Book;

        pub struct Writer {
            pub first_name: String,
            pub last_name: String,
            pub books: Vec<Book>,
        }
    }
}

use library::writers::Writer;

pub fn order_books(writer: &mut Writer) {
    writer.books.sort_by(|a, b| {
        a.title.to_lowercase().cmp(&b.title.to_lowercase())
    });
}
     */