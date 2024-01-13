use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reverse = String::new();

    for c in input.graphemes(true).rev() {
        reverse.push_str(c);
    }

    reverse
}
