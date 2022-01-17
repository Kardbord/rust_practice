use std::ops::Range;

pub fn is_fizz(n: u32) -> bool {
    n % 3 == 0
}

pub fn is_buzz(n: u32) -> bool {
    n % 5 == 0
}

pub fn is_fizzbuzz(n: u32) -> bool {
    is_fizz(n) && is_buzz(n)
}

pub fn fizzbuzz(r: Range<usize>) -> String {
    let mut output = String::new();
    for n in r.start..r.end {
        output = match n {
            _ if is_fizzbuzz(n as u32) => output + "fizzbuzz\n",
            _ if is_fizz(n as u32) => output + "fizz\n",
            _ if is_buzz(n as u32) => output + "buzz\n",
            _ => output + (n as u32).to_string().as_str() + "\n",
        }
    }
    output
}
