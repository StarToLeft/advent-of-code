pub fn parse(split: &str, data: &str) -> Vec<String> {
    let data = data.split(split);
    let mut inputs = vec![];
    for d in data {
        if d == split {
            continue;
        }
        inputs.push(d.to_string());
    }

    return inputs;
}

pub fn upper_half(lower: f32, upper: f32) -> (i32, i32) {
    return (
        (upper - ((upper - lower) / 2.).floor()) as i32,
        upper as i32,
    );
}

pub fn lower_half(lower: f32, upper: f32) -> (i32, i32) {
    return (
        lower as i32,
        (upper - ((upper - lower) / 2.).floor()) as i32 - 1,
    );
}

pub fn count_string(input: &Vec<String>, c: &str) -> u32 {
    let mut count = 0;

    for i in input {
        if i.contains(c) {
            count += 1;
        }
    }

    return count;
}

pub fn count<T: PartialEq>(input: &Vec<T>, c: &T) -> u32 {
    let mut count = 0;

    for i in input {
        if i == c {
            count += 1;
        }
    }

    return count;
}

pub fn replace_local_index_string(
    input: &mut Vec<String>,
    y: &str,
    index: u32,
    replace: &str,
) {
    let mut count = 0;
    for x in input.iter_mut() {
        if x.contains(y) {
            if count == index {
                *x = x.replace(y, replace);
            }
            count += 1;
        }
    }
}

pub fn replace_all(input: &mut Vec<String>, y: &str, replace: &str) {
    for x in input.iter_mut() {
        if x.contains(y) {
            *x = x.replace(y, replace);
        }
    }
}

pub fn find_sub(arr: &Vec<i64>, n: i64, sum: i64) -> (bool, Vec<(i64, i64)>) {
    let mut solution = vec![];
    for i in 0..n {
        let mut sum_so_far = 0;

        for j in i..n {
            sum_so_far += arr[j as usize];

            if sum_so_far == sum {
                println!("Between {} {}", i, j);
                solution.push((i, j));
            }
        }
    }

    if solution.is_empty() {
        return (false, vec![]);
    }

    return (true, solution);
}

pub trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            } else {
                break;
            }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            } else {
                break;
            }
        }
        &self[byte_start..byte_end]
    }
}

use std::ptr;

pub unsafe fn prepend_slice<T: Copy>(vec: &mut Vec<T>, slice: &[T]) {
    let len = vec.len();
    let amt = slice.len();
    vec.reserve(amt);

    ptr::copy(vec.as_ptr(),
              vec.as_mut_ptr().offset((amt) as isize),
              len);
    ptr::copy(slice.as_ptr(),
              vec.as_mut_ptr(),
              amt);
    vec.set_len(len + amt);
}