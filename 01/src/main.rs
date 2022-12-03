use std::fs;

// Instance of a calory list
#[derive(PartialEq, Eq, PartialOrd, Ord)] // for the sorting later on
struct CaloryList {
    items: Vec<u32>,
}

// function total_calories that calculates the total calories of that list
impl CaloryList {
    fn total_calories(&self) -> u32 {
        let mut total = 0;
        let items_iterator = self.items.iter();
        for item in items_iterator {
            total += item;
        }
       total 
    }
}

// a list of those lists for storage
struct ListOfCaloryLists {
    lists: Vec<CaloryList>,
}

// function to create a new entry. Rust really forces you to make nice structs man
impl ListOfCaloryLists {
    fn new_entry(&mut self) {
        self.lists.push(CaloryList { items: Vec::new() });
    }
}

// File path since i'm lazy
const PATH_TO_FILE: &str = "input";

fn main() {
    // Read file
    let file_contents= fs::read_to_string(PATH_TO_FILE).expect("Could not read file");

    // Create a new vector which stores all the lists 
    let mut list_of_calory_lists = ListOfCaloryLists{ lists: Vec::new() };

    // also create a starting list
    list_of_calory_lists.new_entry();

    // go through all the lists
    for line in file_contents.lines() {
        
        // empty line means that we need a new list
        if line.is_empty() {
            list_of_calory_lists.new_entry();
            continue;
        }
        
        // get the current list
        let cur = list_of_calory_lists.lists.last_mut();
        if cur != None {
        // read and store the caloric value of the item in the current line
            cur.unwrap().items.push(line.parse().expect("Not a number"));
        }
    }

    // sort all the lists for the highest caloric value
    list_of_calory_lists.lists.sort_by_key(|element| element.total_calories());
    
    let highest: u32 = list_of_calory_lists.lists.last().unwrap().total_calories();
    // output the highest
    println!("{highest}")
}
    
