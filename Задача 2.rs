/// Определяет длину последовательности Коллатца для числа n.
fn collatz_length(mut n: i32) -> u32 {
    let mut count = 0;
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
            count+=1;
        }
        else {
            n = 3 * n + 1;
            count+=1;
        }
    }
    count+=1;
    return count;
    
}
#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}
fn main() {
    println!("Длина: {}", collatz_length(11));
}