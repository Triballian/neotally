use chrono::{Datelike, Timelike, Utc};
struct Tallytracker {
    // all tallies in this stup
    name: String,
    tallies: vect<Tally>   
}

struct Tally {
    name: String,
    counts: vec<count>,
    number: u32
}

struct Count {
    currentcount: u32,
    attributes: vec<attributes>

}

enum Attribute {

}

struct Defaults {

}

// todo
// quick notes
// home owner
// interested in solar
// objection, no time, dont like the look, spouse needs to decide.
// have a profile for each tally

impl Tally {
    fn gettime (&mut self) {
        let now = Utc::now();
        let (is_pm, hour) = now.hour12();
        let (_, year) = now.year_ce();

    }
    fn count (&mut self) -> {
        self.number = self.number + 1; 
        homelookup
        numberopeeps:
    }
}
//impl time

//count

fn main() {
    
}
