

fn main() {
    let data = vec![
    "a".to_string(),
    "b".to_string(),
    "c".to_string(),
    "d".to_string(),
];
println!(
    "test :{:?}",
    data.iter().fold("".to_string(), |s, x| s + &x)
);

let nums = [1, 2, 3, 4, 5];
let result2 = nums.iter().fold(1, |acc, &x| acc *x);
println!("{}", result2);
}