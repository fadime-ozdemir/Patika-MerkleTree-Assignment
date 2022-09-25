// Import Crates
use std::fs;
use sha2::{Sha256, Digest};


fn main(filename: String) -> String {
    // Read Input Data from txt file
    let file_contents:String=fs::read_to_string(&filename).expect("Error at parse");

    // Create vector of strings for leaves
    let vec_str_leaves: Vec<String> = file_contents.lines().into_iter().map(|s:&str| s.to_string()).collect();
    let bottom_leaves_no:u32 = vec_str_leaves[0].parse().unwrap();
    
    let mut data:Vec<String> = vec_str_leaves[1..vec_str_leaves.len()].to_vec();

    // Hash inputs and append to vector
    let mut hashed: Vec<String> = Vec::new(); 
    for _ in 0..bottom_leaves_no {
        hashed = data.iter().map(|x: &String| hash_input(x)).collect();
        data = create_next_level(hashed);

    }

    // Then Write an algorithm that calculates the ROOT
    // Return the root hash as a String
    hash_input(&data[0])
}

// You can use templates below or just remove
// Helper function to create a next leaves level may help you :)
fn create_next_level(current_level: Vec::<String>) -> Vec::<String> {
    let mut new_level: Vec<String> = Vec::new();
    let mut str = String::from("");
    for (index, value) in current_level.iter().enumerate() {
        if index %2 == 0 {
            str = value.clone();
        }
        else {
            str.push_str(&value);
            new_level.push(str.clone());
        }

    }
    new_level
}


// Helper function may help you to hash an input or You can write macro rules
fn hash_input(input: &str) -> String {
   let mut hasher = Sha256::new();
   hasher.update(input);
   let hash= hasher.finalize();
   let hex = hex::encode(&hash);
   
   
  return hex.to_string();

}




// Pass all tests!
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_0() {
        let result = main("input0.txt".to_string());
        assert_eq!(result, "ff41418be11ed77612aeb83ee0bcf97a5853a4c291e23bd4d4cc6435fcfabdf9");
    }

    #[test]
    fn input_1() {
        let result = main("input1.txt".to_string());
        assert_eq!(result, "98a77b2c3ff5f6c2aca697f60b2aa2a1a2733be36dbd35bae23d517c2ad5985e");
    }

    #[test]
    fn input_2() {
        let result = main("input2.txt".to_string());
        assert_eq!(result, "3c0fb0638de91551eae4e9d984d72034aa0693be37b51737e6b81bc489866e5e");
    }

    #[test]
    fn input_3() {
        let result = main("input3.txt".to_string());
        assert_eq!(result, "f03b1c9163babeb728ac011fe0c2c9c69700a2f8ddde211ec07d621cdb322cfe");
    }

    #[test]
    fn input_4() {
        let result = main("input4.txt".to_string());
        assert_eq!(result, "f83e74742fda659dfc07615881af796abafc434f591aeb23b9f4366abe03c597");
    }
}