use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::slice::*;

struct Cin {
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn new() -> Self {
        let tokens = VecDeque::new();
        Self { tokens }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.tokens.is_empty() {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            for s in buffer.split_whitespace() {
                self.tokens.push_back(s.to_string());
            }
        }
        let fr = self.tokens.pop_front().unwrap();
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let s = cin.next::<String>().into_bytes();

        let mut r = 0;
        let mut m = 0;

        for i in 0..s.len() {
            if s[i] == b'R' && r == 0 {
                r = i + 1;
            }
            if s[i] == b'M' && m == 0 {
                m = i + 1;
            }
        }

        if r < m {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
