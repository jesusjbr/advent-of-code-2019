fn check_criteria_part1(number: i32) -> bool {
    let mut previous = '0';
    let mut ascending = true;
    let mut repeated = false;
    for digit in number.to_string().chars() {
        if previous > digit {
            ascending = false;
            break;
        } else if previous == digit {
            repeated = true;
        }
        previous = digit;
    }
    ascending && repeated
}

fn check_criteria_part2(number: i32) -> bool {
    let mut previous = '0';
    let mut ascending = true;
    let mut group_found = false;
    let mut repeated = 1;
    for digit in number.to_string().chars() {
        if previous > digit {
            ascending = false;
            break;
        } else if previous == digit {
            repeated += 1;
        } else {
            if repeated == 2 {
                group_found = true;
            }
            repeated = 1;
        }
        previous = digit;
    }
    ascending && (group_found || repeated == 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let count = (402_328..864_247)
            .filter(|n| check_criteria_part1(*n))
            .count();
        assert_eq!(count, 454);
    }

    #[test]
    fn test_part2() {
        let count = (402_328..864_247)
            .filter(|n| check_criteria_part2(*n))
            .count();
        assert_eq!(count, 288);
    }
}
