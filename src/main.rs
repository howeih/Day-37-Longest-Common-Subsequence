use std::collections::VecDeque;
use std::iter::FromIterator;

fn find_lcs(x: Vec<char>, y: Vec<char>, backtrace: Vec<Vec<(i32, i32)>>) -> VecDeque<char> {
    let mut lcs = VecDeque::<char>::new();
    let mut r = x.len();
    let mut c = y.len();
    while r != 0 && c != 0 {
        if x[r - 1] == y[c - 1] {
            lcs.push_front(x[r - 1]);
        }
        let trace_step = backtrace[r][c];
        r = (r as i32 + trace_step.0) as usize;
        c = (c as i32 + trace_step.1) as usize;
    }
    lcs
}

fn lcs(x: &str, y: &str) -> VecDeque<char> {
    let x: Vec<char> = x.chars().collect();
    let y: Vec<char> = y.chars().collect();
    let mut lcs_table = vec![vec![0; y.len() + 1]; x.len() + 1];
    let mut backtrace = vec![vec![(0, 0); y.len() + 1]; x.len() + 1];
    for i in 1..=x.len() {
        for j in 1..=y.len() {
            if x[i - 1] == y[j - 1] {
                lcs_table[i][j] = lcs_table[i - 1][j - 1] + 1;
                backtrace[i][j] = (-1, -1);
            } else {
                if lcs_table[i - 1][j] >= lcs_table[i][j - 1] {
                    backtrace[i][j] = (-1, 0);
                    lcs_table[i][j] = lcs_table[i - 1][j];
                } else {
                    backtrace[i][j] = (0, -1);
                    lcs_table[i][j] = lcs_table[i][j - 1];
                }
            }
        }
    }
    find_lcs(x, y, backtrace)
}
fn main() {
    let x = "ABCBDAB";
    let y = "BDCABA";
    let lcs = lcs(x, y);
    assert_eq!(Vec::from_iter(lcs.into_iter()), vec!['B', 'C', 'B', 'A']);
}
