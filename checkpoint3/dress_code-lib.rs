#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black,
    White,
    Flowers,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Hat {
    Snapback,
    Baseball,
    Fedora,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}

pub fn choose_outfit(
    formality_level: Option<u32>,
    invitation_message: Result<&str, &str>,
) -> Outfit {
    // We match on both values as a tuple to easily catch the specific override case first
    match (formality_level, invitation_message) {
        // Specific case: formality_level is None and invitation_message is not Ok
        (None, Err(_)) => Outfit {
            jacket: Jacket::Flowers,
            hat: Hat::Baseball,
        },
        // General cases
        (level, msg) => {
            let jacket = match level {
                None => Jacket::Flowers,
                Some(0) => Jacket::Black,
                Some(_) => Jacket::White,
            };

            let hat = match msg {
                Ok(_) => Hat::Fedora,
                Err(_) => Hat::Snapback,
            };

            Outfit { jacket, hat }
        }
    }
}