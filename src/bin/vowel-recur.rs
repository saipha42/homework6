
fn main() {

    let input_string = "ab12Exey5 7x8U3y5z";

    let result = count_vowels(&input_string, 0);


    println!("Number of vowels : {}", result);
}


fn count_vowels(vowel_string: &str , mut result : i32) -> i32 {

    let vw_vec = vec!["a", "e", "i", "o", "u", "A", "E", "I", "o", "U"];

    if vowel_string.is_empty() {
        result
    }else {
        let (vw_char, vw_str) = vowel_string.split_at(1);
        if vw_vec.contains(&vw_char) {
            result += 1;
        }
        count_vowels(vw_str, result)

    }
}



#[test]
fn test_vowels_count1() {
    assert_eq!(count_vowels("", 0), 0);
    assert_eq!(count_vowels("abEcd", 0), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z", 0), 4);

    assert_eq!(count_vowels("ab12Exey5", 0), 3);
    assert_eq!(count_vowels("ab12Exey5 ab12Exey5 aeiou", 0), 11);
}