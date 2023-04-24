use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut s = "".to_owned();
    for c in input.graphemes(true) {
        s = format!("{}{}", c, s);
    }
    s
}
