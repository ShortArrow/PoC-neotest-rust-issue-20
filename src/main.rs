fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    let ans = if y == 0 { None } else { Some(x / y) };
    ans
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test20() {
    let result = func_ex_div_some(10, 5);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), Some(2));
}

#[test]
fn itworks() {
    assert_eq!(2, 2)
}
