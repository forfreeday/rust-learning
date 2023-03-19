use rand;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        let rand_number: u8 = rand::random();
        println!("rand number {}", rand_number);
        assert_eq!(result, 4);
    }
}
