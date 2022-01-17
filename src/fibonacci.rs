use std::fmt;

pub enum Method {
    Recursive,
    Dynamic,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Method::Recursive => write!(f, "Recursive"),
            Method::Dynamic => write!(f, "Dynamic"),
        }
    }
}

pub fn compute(n: u32, method: Method) -> u64 {
    match method {
        Method::Recursive => compute_recursive(n),
        Method::Dynamic => compute_dynamic(n),
    }
}

fn compute_dynamic(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut curr: u64 = 1;
            let mut prev: u64 = 0;
            for _ in 0..(n - 1) as u64 {
                curr = curr + prev;
                prev = curr - prev;
            }
            curr
        }
    }
}

fn compute_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => compute_recursive(n - 1) + compute_recursive(n - 2),
    }
}
