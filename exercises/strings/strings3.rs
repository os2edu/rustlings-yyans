// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // let mut left = 0;
    // let mut right = 0;
    // for i in 0..input.len() {
    //     let c = input[i..i+1];
    //     if c == ' ' {
    //         left = i;
    //         break;
    //     }
    // }
    // for i in input.len()..1 {
    //     let c = input[i..i+1];
    //     if c == ' ' {
    //         right = i;
    //         break;
    //     }
    // }
    // input[left..right].to_string()
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // let s = "cars";
    // // TODO: Replace "cars" in the string with "balloons"!
    // let mut left = 0;
    // let mut right = 0;
    // for i in 0..input.len() {
    //     let flag = true;
    //     for j in i..i+s.len() {
    //         if input[j] != s[j] {
    //             flag = false;
    //             break;
    //         }
    //     }
    //     if flag {
    //         left = i;
    //         right = left + s.len();
    //         break;
    //     }
    // }
    // format!("{}{}{}", input[..left].to_string, "balloons".to_string, input[right+1..])
    input.to_string().replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
