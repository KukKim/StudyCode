mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate :: front_of_house :: hosting; // absolute path
use self :: front_of_house :: hosting; // relative path (not recommend)

// use self :: front_of_house :: hosting :: add_to_waitlist; // not recommend(if it's function)

pub fn eat_at_restaurant() {
    hosting :: add_to_waitlist();
    hosting :: add_to_waitlist();
    hosting :: add_to_waitlist();
}