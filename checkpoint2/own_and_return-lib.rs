pub struct Film {
    pub name: String,
}

pub fn read_film_name(film: &Film) -> String {
    film.name.clone()
}

pub fn take_film_name(film: Film) -> String {
    film.name
}


/*
pub struct Film {
    pub name: String,
}

/// Borrows the film (uses `&Film`). The caller retains ownership of the `Film`.
/// Because we must return an owned `String`, we have to `.clone()` the name.
pub fn read_film_name(film: &Film) -> String {
    film.name.clone()
}

/// Takes ownership of the film (uses `Film`). The caller loses access to the `Film`.
/// We extract and return the `String` directly, which consumes the `Film` struct.
pub fn take_film_name(film: Film) -> String {
    film.name
}
*/