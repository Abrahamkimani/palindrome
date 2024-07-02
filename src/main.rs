fn main() {
    dbg!(is_palindrome(121));
    dbg!(is_palindrome(-121));
    dbg!(is_palindrome(10));

// solving it by converting it into a string
// fn is_palindrome(x: i32) -> bool {
//     x.to_string() == x.to_string().chars().rev().collect::<String>()

// }

//solving it without as an integer without converting into a string

pub fn is_palindrome(x: i32) -> bool{
    if x< 0{
        false
    }else {
        let (mut rev, mut num) = (0_i32, x);
        while num > 0 {
            rev = rev*10 + num % 10;
            num/= 10;
        }
        rev == x
    }
    
}

}

// num = 495, rev = 0
//1: rev = 0*10 + 495 % 10
//1: rev = 5, num = 49

//2: rev = 5*10 + 49% 10
//2: rev = 59, num = 4.

//3: rev = 59* 10 + 4 % 10
//3: rev = 594, num = 0
