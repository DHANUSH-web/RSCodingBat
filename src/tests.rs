use super::*;

#[test]
fn test_two_char() {
    assert_eq!(two_char("java", 0), "ja");
    assert_eq!(two_char("java", 2), "va");
    assert_eq!(two_char("java", 3), "ja");
}

#[test]
fn test_middle_three() {
    assert_eq!(middle_three("Candy"), "and");
    assert_eq!(middle_three("and"), "and");
    assert_eq!(middle_three("solving"), "lvi");
}

#[test]
fn test_has_bad() {
    assert!(has_bad("badxx"));
    assert!(has_bad("xbadxx"));
    assert!(!has_bad("xxbadxx"));
}

#[test]
fn test_at_first() {
    assert_eq!(at_first("hello"), "he");
    assert_eq!(at_first("hi"), "hi");
    assert_eq!(at_first("h"), "h@");
    assert_eq!(at_first(""), "@@");
}

#[test]
fn test_last_chars() {
    assert_eq!(last_chars("last", "chars"), "ls");
    assert_eq!(last_chars("yo", "java"), "ya");
    assert_eq!(last_chars("hi", ""), "h@");
}

#[test]
fn test_concat() {
    assert_eq!(concat("abc", "cat"), "abcat");
    assert_eq!(concat("dog", "cat"), "dogcat");
    assert_eq!(concat("abc", ""), "abc");
}

#[test]
fn test_last_two() {
    assert_eq!(last_two("coding"), "codign");
    assert_eq!(last_two("cat"), "cta");
    assert_eq!(last_two("ab"), "ba");
}

#[test]
fn test_front_again() {
    assert!(front_again("edited"));
    assert!(!front_again("edit"));
    assert!(front_again("ed"));
}

#[test]
fn test_tea_party() {
    assert_eq!(tea_party(6, 8), 1);
    assert_eq!(tea_party(3, 8), 0);
    assert_eq!(tea_party(20, 6), 2);
}

#[test]
fn test_fizz_string() {
    assert_eq!(fizz_string("fig"), "Fizz");
    assert_eq!(fizz_string("dib"), "Buzz");
    assert_eq!(fizz_string("fib"), "FizzBuzz");
    assert_eq!(fizz_string("hello"), "hello");
}

#[test]
fn test_fizz_string2() {
    assert_eq!(fizz_string2(1), "1!");
    assert_eq!(fizz_string2(2), "2!");
    assert_eq!(fizz_string2(3), "Fizz!");
}

#[test]
fn test_two_as_one() {
    assert!(two_as_one(1, 2, 3));
    assert!(two_as_one(3, 1, 2));
    assert!(!two_as_one(3, 2, 2));
}

#[test]
fn test_in_order() {
    assert!(in_order(1, 2, 4, false));
    assert!(!in_order(1, 2, 1, false));
    assert!(in_order(1, 1, 2, true));
}

#[test]
fn test_in_order_equal() {
    assert!(in_order_equal(2, 5, 11, false));
    assert!(!in_order_equal(5, 7, 6, false));
    assert!(in_order_equal(5, 5, 7, true));
}

#[test]
fn test_last_digit() {
    assert!(last_digit(12, 34, 22));
    assert!(!last_digit(12, 34, 57));
    assert!(last_digit(12, 34, 2));
}

#[test]
fn test_less_by_10() {
    assert!(less_by_10(1, 2, 11));
    assert!(!less_by_10(3, 11, 12));
    assert!(less_by_10(12, 1, 11));
}

#[test]
fn test_without_doubles() {
    assert_eq!(without_doubles(2, 3, true), 5);
    assert_eq!(without_doubles(3, 3, true), 7);
    assert_eq!(without_doubles(3, 3, false), 6);
}

#[test]
fn test_max_mod5() {
    assert_eq!(max_mod5(2, 3), 3);
    assert_eq!(max_mod5(6, 2), 6);
    assert_eq!(max_mod5(3, 2), 3);
}

#[test]
fn test_red_ticket() {
    assert_eq!(red_ticket(2, 2, 2), 10);
    assert_eq!(red_ticket(2, 2, 1), 0);
    assert_eq!(red_ticket(0, 0, 0), 5);
}

#[test]
fn test_green_ticket() {
    assert_eq!(green_ticket(1, 2, 3), 0);
    assert_eq!(green_ticket(2, 2, 2), 20);
    assert_eq!(green_ticket(1, 1, 2), 10);
}

#[test]
fn test_two_char_unicode() {
    assert_eq!(two_char("ábcd", 0), "áb");
    assert_eq!(two_char("ábcd", 2), "cd");
    assert_eq!(two_char("ábcd", 9), "áb");
}

#[test]
fn test_has_bad_unicode_prefix_offset() {
    assert!(has_bad("😊badxx"));
    assert!(!has_bad("😊xbadxx"));
}

#[test]
fn test_last_two_unicode() {
    assert_eq!(last_two("a😊"), "😊a");
    assert_eq!(last_two("éßç"), "éçß");
}

#[test]
fn test_front_again_unicode() {
    assert!(front_again("éxéx"));
    assert!(!front_again("éxyx"));
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
}

#[test]
fn test_sum13() {
    assert_eq!(sum13(&[1, 2, 2, 1]), 6);
    assert_eq!(sum13(&[1, 1]), 2);
    assert_eq!(sum13(&[1, 2, 2, 1, 13]), 6);
}

#[test]
fn test_centered_average() {
    assert_eq!(centered_average(&[1, 2, 3, 4, 100]), 3);
    assert_eq!(centered_average(&[1, 1, 5, 5, 10, 8, 7]), 5);
    assert_eq!(centered_average(&[-10, -4, -2, -4, -2, 0]), -3);
}

#[test]
fn test_sum67() {
    assert_eq!(sum67(&[1, 2, 2]), 5);
    assert_eq!(sum67(&[1, 2, 2, 6, 99, 99, 7]), 5);
    assert_eq!(sum67(&[1, 1, 6, 7, 2]), 4);
    assert_eq!(sum67(&[6, 7, 2]), 2);
    assert_eq!(sum67(&[1, 6, 7, 6, 7]), 1);
    assert_eq!(sum67(&[1, 6, 7, 2, 6, 99, 7]), 3);
    assert_eq!(sum67(&[1, 6, 99, 7, 7]), 8);
    assert_eq!(sum67(&[6, 7, 7]), 7);
}

#[test]
fn test_has22() {
    assert!(has22(&[1, 2, 2]));
    assert!(!has22(&[1, 2, 1, 2]));
    assert!(!has22(&[1, 1, 2]));
}

#[test]
fn test_lucky13() {
    assert!(lucky13(&[0, 2, 4]));
    assert!(!lucky13(&[1, 2, 3]));
    assert!(!lucky13(&[1, 2, 4]));
}

#[test]
fn test_sum28() {
    assert!(sum28(&[2, 3, 2, 2, 4, 2]));
    assert!(!sum28(&[2, 3, 2, 2, 4, 2, 2]));
    assert!(!sum28(&[1, 2, 3, 4]));
}

#[test]
fn test_more14() {
    assert!(more14(&[1, 4, 1]));
    assert!(!more14(&[1, 4, 1, 4]));
    assert!(more14(&[1, 1]));
}

#[test]
fn test_fizz_array() {
    assert_eq!(fizz_array(4), &[0, 1, 2, 3]);
    assert_eq!(fizz_array(1), &[0]);
    assert_eq!(fizz_array(10), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_only14() {
    assert!(only14(&[1, 4, 1, 4]));
    assert!(!only14(&[1, 4, 2, 4]));
    assert!(only14(&[1, 1]));
}

#[test]
fn test_fizz_array2() {
    assert_eq!(fizz_array2(4), ["0", "1", "2", "3"]);
    assert_eq!(fizz_array2(1), ["0"]);
    assert_eq!(fizz_array2(10), ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);
}

#[test]
fn test_no14() {
    assert!(no14(&[1, 2, 3]));
    assert!(!no14(&[1, 2, 3, 4]));
    assert!(no14(&[2, 3, 4]));
}

#[test]
fn test_is_everywhere() {
    assert!(is_everywhere(&[1, 2, 1, 3], 1));
    assert!(!is_everywhere(&[1, 2, 1, 3], 2));
    assert!(!is_everywhere(&[1, 2, 1, 3, 4], 1));
}

// #[tokio::test]
// async fn test_fetch_json_data() -> Result<(), reqwest::Error> {
//     let url: &str = "https://jsonplaceholder.typicode.com/todos/1";
//     let data: IJsonProps = fetch_json_data(url).await?;

//     assert_eq!(data.userId, 1);
//     assert_eq!(data.id, 1);
//     assert_eq!(data.title, "delectus aut autem");
//     assert!(!data.completed);

//     Ok(())
// }
