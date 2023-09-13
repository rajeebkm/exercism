use unicode_segmentation::UnicodeSegmentation;

fn reverse_string(input: &str) -> String {
    UnicodeSegmentation::graphemes(input, true).rev().collect::<String>()
    // input.chars().rev().collect::<String>()
}

fn main() {
    let input_string = "cool";
    println!("{}", reverse_string(input_string));
    let input_string = "uüu";
    println!("{}", reverse_string(input_string));
}

