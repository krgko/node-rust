extern crate libc;

use libc::c_char;
use std::ffi::CString;

pub fn fizzbuzz_base(n: i32) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        return String::from("fizzbuzz");
    } else if n % 3 == 0 {
        return String::from("fizz");
    } else if n % 5 == 0 {
        return String::from("buzz");
    } else {
        return String::from(format!("{}", n.to_string()));
    }
}

#[no_mangle]
pub extern "C" fn fizzbuzz(n: i32) -> *mut c_char {
    let mut result = "".to_string();
    for i in 1..=n {
        if i > 1 {
            result = format!(
                "{}",
                result.to_string() + &",".to_string() + &*fizzbuzz_base(i)
            );
        } else {
            result = format!("{}", result.to_string() + &*fizzbuzz_base(i));
        }
    }

    let c_str_result = CString::new(result).unwrap();
    return c_str_result.into_raw();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_base_put_3() {
        assert_eq!(fizzbuzz_base(3), "fizz")
    }

    #[test]
    fn fizzbuzz_base_put_5() {
        assert_eq!(fizzbuzz_base(5), "buzz")
    }

    #[test]
    fn fizzbuzz_base_put_11() {
        assert_eq!(fizzbuzz_base(11), "11")
    }

    #[test]
    fn fizzbuzz_put_3() {
        unsafe {
            assert_eq!(
                CString::from_raw(fizzbuzz(3)),
                CString::new("1,2,fizz").unwrap()
            )
        }
    }

    #[test]
    fn fizzbuzz_put_5() {
        unsafe {
            assert_eq!(
                CString::from_raw(fizzbuzz(5)),
                CString::new("1,2,fizz,4,buzz").unwrap()
            )
        }
    }

    #[test]
    fn fizzbuzz_put_15() {
        unsafe {
            assert_eq!(
                CString::from_raw(fizzbuzz(15)),
                CString::new("1,2,fizz,4,buzz,fizz,7,8,fizz,buzz,11,fizz,13,14,fizzbuzz").unwrap()
            )
        }
    }

    #[test]
    fn fizzbuzz_put_100() {
        unsafe {
            assert_eq!(
                CString::from_raw(fizzbuzz(100)),
                CString::new("1,2,fizz,4,buzz,fizz,7,8,fizz,buzz,11,fizz,13,14,fizzbuzz,16,17,fizz,19,buzz,fizz,22,23,fizz,buzz,26,fizz,28,29,fizzbuzz,31,32,fizz,34,buzz,fizz,37,38,fizz,buzz,41,fizz,43,44,fizzbuzz,46,47,fizz,49,buzz,fizz,52,53,fizz,buzz,56,fizz,58,59,fizzbuzz,61,62,fizz,64,buzz,fizz,67,68,fizz,buzz,71,fizz,73,74,fizzbuzz,76,77,fizz,79,buzz,fizz,82,83,fizz,buzz,86,fizz,88,89,fizzbuzz,91,92,fizz,94,buzz,fizz,97,98,fizz,buzz").unwrap()
            )
        }
    }
}
