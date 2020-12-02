#[derive(Debug)]
struct Entry {
    first: usize,
    second: usize,
    c: char,
    password: String,
}

fn main() -> anyhow::Result<()> {

    let input = std::fs::read_to_string("./src/day2/input.txt")?;
    let input: Vec<Entry> = input.lines().map(|line| {
        let mut parts =  line.split_ascii_whitespace();
        let part1 = parts.next().expect("input in wrong format");
        let part2 = parts.next().expect("input in wrong format");
        let part3 = parts.next().expect("input in wrong format");

        let mut range = part1.split('-');
        let min: usize = range.next().expect("input in wrong format").parse().expect("NAN");
        let max: usize = range.next().expect("input in wrong format").parse().expect("NAN");

        let character = part2.chars().nth(0).unwrap();

        let password = part3.to_string();

        Entry {
            first: min,
            second: max,
            c: character,
            password,
        }
    }).collect::<Vec<_>>();

    let count = input.iter().map(|e| validate_entry_policy1(e)).filter(|b| *b).count();
    println!("Valid Passwords Policy 1: {}", count);

    let count = input.iter().map(|e| validate_entry_policy2(e)).filter(|b| *b).count();
    println!("Valid Passwords Policy 2: {}", count);
    Ok(())
}

fn validate_entry_policy1(entry: &Entry) -> bool {
    let instances = entry.password.chars().filter(|c| *c == entry.c).count();
    return instances >= entry.first as usize && instances <= entry.second as usize;
}

fn validate_entry_policy2(entry: &Entry) -> bool {
    let v: Vec<char> = entry.password.chars().collect();

    // Out of bounds check!
    if entry.first > entry.password.len() || entry.second > entry.password.len() {
        return false;
    }

    let r1 = v[entry.first - 1] == entry.c;
    let r2 = v[entry.second - 1] == entry.c;
    return (r1 || r2) && !(r1 && r2);
}

#[cfg(test)]
mod test {
    use crate::{validate_entry_policy1, Entry, validate_entry_policy2};

    #[test]
    fn test1() {
        let e = Entry {
            first: 1,
            second: 3,
            c: 'b',
            password: "cdefg".to_string(),
        };
        assert!(! validate_entry_policy1(&e));
    }

    #[test]
    fn test2() {
        let e = Entry {
            first: 1,
            second: 3,
            c: 'a',
            password: "abcde".to_string(),
        };
        assert!(validate_entry_policy1(&e));
    }

    #[test]
    fn test3() {
        let e = Entry {
            first: 2,
            second: 3,
            c: 'c',
            password: "cccccccc".to_string(),
        };
        assert!(! validate_entry_policy1(&e));
    }

    #[test]
    fn test4() {
        let e = Entry {
            first: 1,
            second: 3,
            c: 'b',
            password: "cdefg".to_string(),
        };
        assert!(! validate_entry_policy2(&e));
    }

    #[test]
    fn test5() {
        let e = Entry {
            first: 1,
            second: 3,
            c: 'a',
            password: "abcde".to_string(),
        };
        assert!(validate_entry_policy2(&e));
    }

    #[test]
    fn test6() {
        let e = Entry {
            first: 2,
            second: 3,
            c: 'c',
            password: "cccccccc".to_string(),
        };
        assert!(! validate_entry_policy2(&e));
    }
}