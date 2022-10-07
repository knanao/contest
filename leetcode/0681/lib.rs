struct Solution;
impl Solution {
    // H1H2:M1M2
    // H1: 0-2
    // H2: 0-9 (0-3 if H1==2)
    // M1: 0-5
    // M2: 0-9
    pub fn next_closest_time(time: String) -> String {
        let cs: Vec<char> = time.chars().collect();
        let mut digits = vec![cs[0], cs[1], cs[3], cs[4]];
        let mut ret = cs.clone();
        digits.sort();

        ret[4] = Self::find_next(ret[4], '9', &digits);
        if ret[4] > cs[4] {
            return String::from_iter(ret);
        }

        ret[3] = Self::find_next(ret[3], '5', &digits);
        if ret[3] > cs[3] {
            return String::from_iter(ret);
        }

        if ret[0] == '2' {
            ret[1] = Self::find_next(ret[1], '3', &digits);
        } else {
            ret[1] = Self::find_next(ret[1], '9', &digits);
        }
        if ret[1] > cs[1] {
            return String::from_iter(ret);
        }

        ret[0] = Self::find_next(ret[0], '2', &cs);
        return String::from_iter(ret);
    }

    fn find_next(curr: char, upper: char, digits: &Vec<char>) -> char {
        if curr == upper {
            return digits[0];
        }

        let mut pos = digits.binary_search(&curr).unwrap() + 1;
        while pos < 4 && (digits[pos] > upper || digits[pos] == curr) {
            pos += 1;
        }

        if pos == 4 {
            digits[0]
        } else {
            digits[pos]
        }
    }
}
