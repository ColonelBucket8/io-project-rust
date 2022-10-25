use std::env;

fn main() {
    // collect is one function you do often need to 
    // annotate because Rust isn't able to infer the kind
    // of collection you want
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
    dbg!("query: {}, file path: {}", query, file_path);
}
