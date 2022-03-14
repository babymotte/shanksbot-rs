use std::collections::HashSet;

pub fn shanks(number: u64) -> u64 {
    let mut known_remainders = HashSet::new();

    let mut counter = 0u64;
    let mut remainder = 1;

    while {
        while number > remainder {
            counter += 1;
            remainder *= 10;
            known_remainders.insert(remainder);
        }
        known_remainders.insert(remainder);
        let div = remainder / number;
        remainder = (remainder - (div * number)) * 10;
        remainder != 0 && !known_remainders.contains(&remainder)
    } {
        counter += 1;
    }

    counter
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn shanks1() {
        assert_eq!(shanks(1), 0);
    }

    #[test]
    fn shanks2() {
        assert_eq!(shanks(2), 1);
    }

    #[test]
    fn shanks3() {
        assert_eq!(shanks(3), 1);
    }

    #[test]
    fn shanks5() {
        assert_eq!(shanks(5), 1);
    }

    #[test]
    fn shanks7() {
        assert_eq!(shanks(7), 6);
    }

    #[test]
    fn shanks23() {
        assert_eq!(shanks(23), 22);
    }

    #[test]
    fn shanks60013() {
        assert_eq!(shanks(60013), 5001);
    }

    #[test]
    fn shanks60017() {
        assert_eq!(shanks(60017), 60016);
    }

    #[test]
    fn shanks60037() {
        assert_eq!(shanks(60037), 10006);
    }

    #[test]
    fn shanks61561() {
        assert_eq!(shanks(61561), 405);
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn sanity() {
        for i in 50_000..70_000 {
            assert!(shanks(i) < i);
        }
    }
}
