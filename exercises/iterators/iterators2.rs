// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            let mut a:String=first.to_uppercase().to_string();
            let mut b:String=c.collect();
            format!("{}{}", a, b)
        
            
            
            
        },
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut word : Vec<String>  = vec![];
    let mut chars  = words.iter();
    while let Some(str_1) = chars.next() {
        let mut c = str_1.chars();
        match c.next() {
            None => {}
            Some(first) => {
                let mut a:String=first.to_uppercase().to_string();
                let mut b:String=c.collect();
                
                word.push(format!("{}{}", a, b));
                
            },
        }
    }
    word

}
// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut word :String = String::from("") ;
    let mut chars  = words.iter();
    while let Some(str_1) = chars.next() {
        let mut c = str_1.chars();
        match c.next() {
            None => {}
            Some(first) => {
                if first == ' '{
                    word+=" ";
                }else{
                    let mut a:String=first.to_uppercase().to_string();
                    let mut b:String=c.collect();
                    word = format!("{}{}{}",word, a, b);
                }
                
                
                
                
            },
        }
    }
    word

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
