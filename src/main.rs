use std::{collections::HashMap, fs::read_to_string}; //  Makes HashMap type available as "HashMap"

fn main() {
    // args() returns an iterator over the cli passed arguments
    // skip() from the iterator skips number of elments given; here the first one
    // which would be the name of the cli programm.
    // args is still an iterator though!
    let mut args = std::env::args().skip(1);

    // next() of an iterator returns an Option; meaning there either is a next element or not
    // expect() either returns the element or the program dies with the message given
    let key = args.next().expect("Key was not given");

    let value = args.next().expect("Value was not given");
    // unwrap returns the Ok from next()'s Option as well, but just dies without a message
    // let value = args.next().unwrap();

    println!("The key is {} the value is {}", key, value);

    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();

    // important: expect() is what will crash the program if an error is return!
    // meaning if expect is not called and an error is return -> database _is_ an error
    let database = Database::new().expect("Failed to create Database");

}

// A struct defines the type and its fields
struct Database {
    map: HashMap<String, String>
}

// An implementation adds funtionality to a given Type
impl Database  {
    // new creates a new Database instance
    // new is not special! Rust does not know "constructors" its just a convention to use new
    fn new() -> Result<Database, std::io::Error> {
        // What does new need to do?
        //  - read kv.db file
        //  - populate map
        //  - return Database instance

        // match is an expression, not a statement, so we can bind the value of the expression
        // the whole thing below is so common in rust that there is a shortcut for it which is -> ?
        // meaning, every thime i want to call a function that either returns a value or and error
        // i can use the ? operator to either return the value or bubble up the error to the caller
        // the below code with the ? operator would be:
        //     let contents = std::fs::read_to_string("kv.db")?;
        //
        //  for example read_to_string will return an error, if the file does not exist!
        let contents = match std::fs::read_to_string("kv.db") {
            // Just Return the content and bind value to contents
            Ok(c) => c,
            Err (error) => {
                // Returning an error, forces to return a Result from new()
                // But you have to return a Err() of the error ... !?
                return Err(error);
            }
        };

        // Wrap databse instance in Ok() to return a valid Result<>
        Ok(Database {
            map: HashMap::new()
        })
    }
}
