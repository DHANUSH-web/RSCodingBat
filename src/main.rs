// use serde::{Deserialize, Serialize};
// use std::process::Output;

pub fn two_char(msg: &str, index: i32) -> &str {
    let mut starts: Vec<usize> = msg.char_indices().map(|(i, _)| i).collect();
    starts.push(msg.len());

    let char_len = starts.len() - 1;

    if char_len <= 2 {
        return msg;
    }

    let start_char = if index >= 0 && (index as usize) + 1 < char_len {
        index as usize
    } else {
        0
    };
    let start = starts[start_char];
    let end = starts[start_char + 2];

    &msg[start..end]
}

pub fn middle_three(msg: &str) -> &str {
    let mut starts: Vec<usize> = msg.char_indices().map(|(i, _)| i).collect();
    starts.push(msg.len());
    let char_len = starts.len() - 1;

    if char_len <= 3 {
        msg
    } else {
        let mid = char_len / 2;
        &msg[starts[mid - 1]..starts[mid + 2]]
    }
}

pub fn has_bad(msg: &str) -> bool {
    if msg.starts_with("bad") {
        return true;
    }

    let mut chars = msg.chars();

    if chars.next().is_none() {
        return false;
    }

    chars.as_str().starts_with("bad")
}

pub fn at_first(msg: &str) -> String {
    msg.chars().chain(std::iter::repeat('@')).take(2).collect()
}

pub fn last_chars(a: &str, b: &str) -> String {
    let first = a.chars().next().unwrap_or('@');
    let last = b.chars().last().unwrap_or('@');
    format!("{}{}", first, last)
}

pub fn concat(a: &str, b: &str) -> String {
    if a.is_empty() {
        return b.to_string();
    }
    if b.is_empty() {
        return a.to_string();
    }

    if a.ends_with(b.chars().next().unwrap()) {
        format!("{}{}", a, &b[b.chars().next().unwrap().len_utf8()..])
    } else {
        format!("{}{}", a, b)
    }
}

pub fn last_two(data: &str) -> String {
    let mut chars: Vec<char> = data.chars().collect();
    if chars.len() < 2 {
        return data.to_string();
    }
    let len = chars.len();
    chars.swap(len - 2, len - 1);
    chars.into_iter().collect()
}

pub fn front_again(data: &str) -> bool {
    let chars: Vec<char> = data.chars().collect();
    let len = chars.len();
    len >= 2 && chars[0] == chars[len - 2] && chars[1] == chars[len - 1]
}

pub fn tea_party(tea: i32, candy: i32) -> i32 {
    if tea < 5 || candy < 5 {
        0
    } else if tea >= candy * 2 || candy >= tea * 2 {
        2
    } else {
        1
    }
}

pub fn fizz_string(data: &str) -> String {
    let f = data.starts_with('f');
    let b = data.ends_with('b');
    if f && b {
        "FizzBuzz".to_string()
    } else if f {
        "Fizz".to_string()
    } else if b {
        "Buzz".to_string()
    } else {
        data.to_string()
    }
}

pub fn fizz_string2(n: i32) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        "FizzBuzz!".to_string()
    } else if n % 3 == 0 {
        "Fizz!".to_string()
    } else if n % 5 == 0 {
        "Buzz!".to_string()
    } else {
        format!("{}!", n)
    }
}

pub fn two_as_one(a: i32, b: i32, c: i32) -> bool {
    a + b == c || a + c == b || b + c == a
}

pub fn in_order(a: i32, b: i32, c: i32, b_ok: bool) -> bool {
    (b_ok || b > a) && c > b
}

pub fn in_order_equal(a: i32, b: i32, c: i32, equal_ok: bool) -> bool {
    (equal_ok && a <= b && b <= c) || (a < b && b < c)
}

pub fn last_digit(a: i32, b: i32, c: i32) -> bool {
    a % 10 == b % 10 || b % 10 == c % 10 || a % 10 == c % 10
}

pub fn less_by_10(a: i32, b: i32, c: i32) -> bool {
    (a - b).abs() >= 10 || (b - c).abs() >= 10 || (a - c).abs() >= 10
}

pub fn without_doubles(mut die1: i32, die2: i32, no_doubles: bool) -> i32 {
    if no_doubles && die1 == die2 {
        die1 = if die1 == 6 { 1 } else { die1 + 1 };
    }

    die1 + die2
}

pub fn max_mod5(a: i32, b: i32) -> i32 {
    if a == b {
        return 0;
    }

    if a % 5 == b % 5 {
        return a.min(b);
    }

    a.max(b)
}

pub fn red_ticket(a: i32, b: i32, c: i32) -> i32 {
    if a == 2 && b == 2 && c == 2 {
        return 10;
    }

    if a == b && b == c {
        return 5;
    }

    if a != b && a != c {
        return 1;
    }

    0
}

pub fn green_ticket(a: i32, b: i32, c: i32) -> i32 {
    if a == b && b == c {
        return 20;
    }

    if a == b || a == c || b == c {
        return 10;
    }

    0
}

// #[allow(non_snake_case)]
// #[derive(Deserialize, Serialize)]
// pub struct IJsonProps {
//     userId: i32,
//     id: i32,
//     title: String,
//     completed: bool,
// }

// pub async fn fetch_json_data(url: &str) -> Result<IJsonProps, reqwest::Error> {
//     reqwest::get(url).await?.json::<IJsonProps>().await
// }

pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    let distance: u32 = end - start;
    distance / time_elapsed
}

pub fn factorial(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    n * factorial(n - 1)
}

// pub fn run_shell(command: &str) -> Result<String, std::io::Error> {
//     let output: Output = std::process::Command::new("bash")
//         .arg("-c")
//         .arg(command)
//         .output()?;
//     String::from_utf8(output.stdout)
//         .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
// }

#[allow(unused_assignments)]
pub fn sum13(nums: &[i32]) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    if nums.len() == 1 && nums[0] == 13 {
        return 0;
    }

    let mut sum: i32 = 0;

    for mut i in 0..nums.len() {
        if nums[i] == 13 {
            i += 1;
            continue;
        }

        sum += nums[i];
    }

    sum
}

pub fn centered_average(nums: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    let mut min: i32 = nums[0];
    let mut max: i32 = nums[0];

    for i in 0..nums.len() {
        sum += nums[i];

        if nums[i] < min {
            min = nums[i];
        }

        if nums[i] > max {
            max = nums[i];
        }
    }

    (sum - min - max) / (nums.len() as i32 - 2)
}

pub fn sum67(nums: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    let mut skip: bool = false;

    for i in 0..nums.len() {
        if nums[i] == 6 {
            skip = true;
        } else if nums[i] == 7 && skip {
            skip = false;
        } else if !skip {
            sum += nums[i];
        }
    }

    sum
}

pub fn has22(nums: &[i32]) -> bool {
    for i in 0..nums.len() - 1 {
        if nums[i] == 2 && nums[i + 1] == 2 {
            return true;
        }
    }

    false
}

pub fn lucky13(nums: &[i32]) -> bool {
    for i in 0..nums.len() {
        if nums[i] == 1 || nums[i] == 3 {
            return false;
        }
    }

    true
}

pub fn sum28(nums: &[i32]) -> bool {
    let mut sum: i32 = 0;

    for i in 0..nums.len() {
        if nums[i] == 2 {
            sum += nums[i];
        }
    }

    sum == 8
}

pub fn more14(nums: &[i32]) -> bool {
    let mut count: i32 = 0;

    for i in 0..nums.len() {
        if nums[i] == 1 {
            count += 1;
        }
        if nums[i] == 4 {
            count -= 1;
        }
    }

    count > 0
}

pub fn fizz_array(n: i32) -> Vec<i32> {
    let mut arr: Vec<i32> = Vec::new();

    for i in 0..n {
        arr.push(i);
    }

    arr
}

pub fn only14(nums: &[i32]) -> bool {
    for i in 0..nums.len() {
        if nums[i] != 1 && nums[i] != 4 {
            return false;
        }
    }

    true
}

pub fn fizz_array2(n: i32) -> Vec<String> {
    let mut arr: Vec<String> = Vec::new();

    for i in 0..n {
        arr.push(i.to_string());
    }

    arr
}

pub fn no14(nums: &[i32]) -> bool {
    let mut has1: bool = false;
    let mut has4: bool = false;

    for i in 0..nums.len() {
        if nums[i] == 1 { has1 = true; }
        if nums[i] == 4 { has4 = true; }
        if has1 && has4 { return false; }
    }

    !(has1 && has4)
}

pub fn is_everywhere(nums: &[i32], val: i32) -> bool {
    for i in 0..nums.len() - 1  {
        if nums[i] != val && nums[i+1] != val {
            return false;
        }
    }

    true
}

pub fn either24(nums: &[i32]) -> bool {
  let mut is2 = false;
  let mut is4 = false;

  for i in 0..nums.len()-1 {
    if is2 && is4 { return false; }
    if nums[i] == 2 && nums[i+1] == 2 { is2 = true; }
    if nums[i] == 4 && nums[i+1] == 4 { is4 = true; }
  }

  is2 != is4
}

pub fn match_up(nums1: &[i32], nums2: &[i32]) ->  i32 {
    let mut count: i32 = 0;

    for i in 0..nums1.len() {
        if nums1[i] != nums2[i] && (nums1[i] - nums2[i]).abs() <= 2 {
            count += 1;
        }
    }

    count
}

pub fn has77(nums: &[i32]) -> bool {
    for i in 0..nums.len()-2 {
        if (nums[i] == 7 && (nums[i+1] == 7 || nums[i+2] == 7)) || (nums[i+1] == 7 && nums[i+2] == 7) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests;

fn main() {
    println!("Run 'cargo test' to run the unit tests.");
}
