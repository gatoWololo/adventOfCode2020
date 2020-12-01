use std::collections::HashSet;
use anyhow::{Result};

fn main() -> Result<()>{
    let input = std::fs::read_to_string("./src/day1/input.txt")?;
    let input = input.split_ascii_whitespace();
    let hs: HashSet<i32> = input.map(|s| s.parse::<i32>().unwrap()).collect();

    let (e1, e2) = find_two_entries(&hs, 2020).unwrap();
    println!("Entries: {} * {} = {}", e1, e2, e1 * e2);

    let (e1, e2, e3) = find_three_entries(&hs, 2020);
    println!("Entries: {} * {} * {} = {}", e1, e2, e3, e1 * e2 * e3);

    Ok(())
}

fn find_two_entries(hs: &HashSet<i32>, target: i32) -> Option<(i32, i32)> {
    for e in hs {
        let other = target - e;
        if hs.contains(&other) {
            return Some((*e, other));
        }
    }
    None
}

fn find_three_entries(hs: &HashSet<i32>, target: i32) -> (i32, i32, i32) {
    for e in hs {
        // _other_ becomes the _target_ for sub-problem.
        let other = target - e;
        if let Some((e2, e3)) = find_two_entries(hs, other) {
            return (*e, e2, e3);
        }
    }

    panic!("3-tuple not in input.");
}

#[cfg(test)]
mod test {
    use crate::{find_two_entries, find_three_entries};
    use std::collections::HashSet;
    use anyhow::Result;

    #[test]
    fn test_find_two_entries() -> Result<()> {
        let mut hs = HashSet::new();
        for e in &[1721i32, 979, 366, 299, 675, 1456] {
            hs.insert(*e);
        }
        let (e1, e2) = find_two_entries(&hs, 2020)?;
        assert_eq!(e1 * e2, 514579);
        Ok(())
    }

    #[test]
    fn test_find_three_entries() {
        let mut hs = HashSet::new();
        for e in &[1721i32, 979, 366, 299, 675, 1456] {
            hs.insert(*e);
        }
        let (e1, e2, e3) = find_three_entries(&hs, 2020);
        assert_eq!(e1 * e2 * e3, 241861950);
    }
}