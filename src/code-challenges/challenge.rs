use std::io;
use unicode_segmentation::UnicodeSegmentation;

pub fn run() {
    let mut input_text = String::new();
    let mut input_number = String::new();

    println!("Enter string: ");
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    println!("Enter number: ");
    io::stdin()
        .read_line(&mut input_number)
        .expect("failed to read from stdin");

    let n: i32 = input_number.trim().parse().expect("invalid number");
    let s = input_text.trim();

    let repeated = repeated_string(n, s);
    
    println!("Count {}", repeated);
}

pub fn repeated_string(n: i32, s: &str) -> i32 {
    let size: i32 = s.graphemes(true).count() as i32;
    let repeated = n / size;
    let left = n - (size * repeated);

    let mut count = 0;
    for x in s.chars() {
        if x == 'a' {
            count += 1;
        }
    }

    let mut extra = 0;
    for x in 0..left {
        if s.chars().nth(x as usize).unwrap() == 'a' {
            extra += 1;
        }
    }

    return (repeated * count) + extra;
}


#[test]
fn test_repeated_string(){
    assert_eq!(repeated_string(20, "a"), 20);
    assert_eq!(repeated_string(10, "aba"), 7);
    assert_eq!(repeated_string(15, "coca"), 3);
}

