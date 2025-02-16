// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread;

pub fn sum(slice: &'static [i32]) -> i32 {
    let mid = slice.len() / 2;
    let (part_1, part_2) = slice.split_at(mid);

    let handle_1 = thread::spawn(move || part_1.iter().sum::<i32>());
    let handle_2 = thread::spawn(move || part_2.iter().sum::<i32>());

    // this also works without "move" keyword. Probably because it uses static lifetime, meaning the threads
    // are guaranteed to have valid references.
    // let handle_1 = thread::spawn(|| part_1.iter().sum::<i32>());
    // let handle_2 = thread::spawn(|| part_2.iter().sum::<i32>());

    handle_1.join().unwrap() + handle_2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
