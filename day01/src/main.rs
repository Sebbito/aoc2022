use std::fs;
use std::iter::Iterator;

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

impl ListOfCaloryLists {

    // function to create a new entry. Rust really forces you to make nice structs man
    fn new_entry(&mut self) {
        self.lists.push(CaloryList { items: Vec::new() });
    }

    // function to read a list of calories
    fn read_list(&mut self, file_contents: &str) {
        // go through all the lists
        for line in file_contents.lines() {
            
            // empty line means that we need a new list
            if line.is_empty() {
                self.new_entry();
                continue;
            }
            
            // get the current list
            let cur = self.lists.last_mut();
            if cur != None {
            // read and store the caloric value of the item in the current line
                cur.unwrap().items.push(line.parse().expect("Not a number"));
            }
        }
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

    // read the list
    list_of_calory_lists.read_list(&file_contents);

    // sort all the lists for the highest caloric value
    list_of_calory_lists.lists.sort_by_key(|element| element.total_calories());
    
    // part 1: output the highest
    let highest: u32 = list_of_calory_lists.lists.last().unwrap().total_calories();
    println!("Highest single caloric value: {highest}");

    // part 2: output the 3 highest and add them together
    let top_3_total: u32 = list_of_calory_lists.lists.iter().rev().take(3).map(|list| list.total_calories()).sum();
    println!("Three highest caloric values combined {top_3_total}");

}
    
