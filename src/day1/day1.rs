#[cfg(not(test))]
static FILE: &str = include_str!("./day1.prod");

#[cfg(test)]
static FILE: &str = include_str!("./day1.test");

pub fn day1_1() -> usize {
    let max = FILE
        .split("\n\n")
        .map(|p| p.lines().flat_map(str::parse::<usize>).sum::<usize>())
        .max()
        .unwrap();

    return max;
}

pub fn day1_2() -> usize {
    let mut max = FILE
        .split("\n\n")
        .map(|p| p.lines().flat_map(str::parse::<usize>).sum::<usize>())
        .collect::<Vec<usize>>();

    max.sort_by(|a, b| b.cmp(a));

    return max.into_iter().take(3).sum();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day1_1() {
        assert_eq!(super::day1_1(), 24000);
    }
}
