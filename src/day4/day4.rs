#[cfg(not(test))]
static FILE: &'static str = include_str!("./day4.prod");

#[cfg(test)]
static FILE: &'static str = include_str!("./day4.test");

pub fn day4_1() -> usize {
    FILE.lines()
        .filter(|&line| {
            let (first, second) = line.split_once(",").unwrap();

            let first: Vec<usize> = first
                .split("-")
                .map(|c| c.parse::<usize>().unwrap())
                .collect();

            let first: Vec<usize> = (first[0]..=first[1]).collect();

            let second: Vec<usize> = second
                .split("-")
                .map(|c| c.parse::<usize>().unwrap())
                .collect();

            let second: Vec<usize> = (second[0]..=second[1]).collect();

            return first.iter().all(|item| second.contains(item))
                || second.iter().all(|item| first.contains(item));
        })
        .count()
}

pub fn day4_2() -> usize {
    FILE.lines()
        .filter(|&line| {
            let (first, second) = line.split_once(",").unwrap();

            let first: Vec<usize> = first
                .split("-")
                .map(|c| c.parse::<usize>().unwrap())
                .collect();

            let first: Vec<usize> = (first[0]..=first[1]).collect();

            let second: Vec<usize> = second
                .split("-")
                .map(|c| c.parse::<usize>().unwrap())
                .collect();

            let second: Vec<usize> = (second[0]..=second[1]).collect();

            return first.iter().any(|item| second.contains(item))
                || second.iter().any(|item| first.contains(item));
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day4_1() {
        assert_eq!(day4_1(), 2);
    }
    #[test]
    fn test_day4_2() {
        assert_eq!(day4_2(), 4);
    }
}
