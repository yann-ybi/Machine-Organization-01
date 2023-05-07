use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::str;

fn main() {

    let mut fname_hm: HashMap<String, Vec<String>> = HashMap::new();

    // for every line read there is a vector of &str containing the fingerprint at index 0 and the name at index 1
    // for every vector of &str there is a match to check if the fingerprint is present in the hashmap as a key string.
        // if present we just push the name in the vector of string value of that key
        // if not, we insert the fingerprint as a new key to the hasmap with its name pushed in a new vector as the value of that key

    for line in io::stdin().lock().lines() {
        let l = line.unwrap();
        let fps_names: Vec<&str> = l.splitn(2,' ').collect();   
        
        match fname_hm.get_mut(fps_names[0]){

            Some(ref_name) => {
                ref_name.push(fps_names[1].trim().to_string())
            }
            None => {
                fname_hm.insert(fps_names[0].to_string(), vec![fps_names[1].trim().to_string()]);
            }
        }
    }

    // for every key/value in the hashmap, only if the value has more than one name then print the names
    // for every group of names print a blank new line before printing the next group once the first group has already been printed.

    let mut first = true;
    for pair in &fname_hm {
        if pair.1.len() > 1{
            if !first {
                println!();
            }
            first = false;
            for name in pair.1 {
                println!("{}", name)
            } 
        }
    }
}
