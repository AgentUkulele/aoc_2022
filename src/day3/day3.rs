use std::collections::HashSet;

#[cfg(not(test))]
static FILE: &'static str = include_str!("./day3.prod");

#[cfg(test)]
static FILE: &'static str = include_str!("./day3.test");

pub fn day3_1() -> usize {
    let total = FILE
        .lines()
        .map(|line| {
            let mut shared: HashSet<char> = HashSet::new();
            let len = line.len();
            let first_half = &line[..len / 2];
            let second_half = &line[len / 2..];

            first_half.chars().for_each(|c| {
                if second_half.contains(c) {
                    shared.insert(c);
                }
            });

            shared
                .into_iter()
                .map(|k| {
                    if k.is_uppercase() {
                        (k as u8 - 38) as usize
                    } else {
                        (k as u8 - 96) as usize
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    return total;
}

pub fn day3_2() -> usize {
    let total = FILE
        .lines()
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|s| <&[&str] as TryInto<[&str; 3]>>::try_into(s).unwrap())
        .map(|[first, second, third]| {
            // let (first, second, third) = (chunk[0], chunk[1], chunk[2]);
            let mut badge = None;
            first.chars().for_each(|c| {
                if second.contains(c) && third.contains(c) {
                    badge = Some(c);
                }
            });

            let badge = badge.unwrap();
            if badge.is_uppercase() {
                (badge as u8 - 38) as usize
            } else {
                (badge as u8 - 96) as usize
            }
        })
        .sum::<usize>();

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day3_1() {
        assert_eq!(day3_1(), 157);
    }

    #[test]
    fn test_day3_2() {
        assert_eq!(day3_2(), 70);
    }
}
