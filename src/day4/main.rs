use anyhow::{Context, ensure, bail};
use std::collections::{HashSet, HashMap};


fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("./src/day4/input.txt").context("Cannot read input")?;

    let entries = parse_input(&input);

    let c = entries.iter().filter(|e| is_valid_entry_part1(e)).count();
    println!("Valid passports part 1 = {}", c);

    let c = entries.iter().filter(|e| is_valid_entry_part2(e)).count();
    println!("Valid passports part2 = {}", c);
    Ok(())
}

pub fn parse_input(input: &str) -> Vec<HashMap<&str, &str>>{
    input.
        // Separate different passports.
        split("\n\n").
        // Separate entries in passports
        map(|s| {
            let fields = s.split_ascii_whitespace();
            // Separate into individual passport field keys.
            fields.fold(HashMap::new(), |mut hs, item| {
                let mut split = item.split(':');
                let e1: &str = split.next().unwrap();
                let e2: &str = split.next().unwrap();
                hs.insert(e1, e2);
                hs
            })
        }).collect()
}

fn is_valid_entry_part1(hs: &HashMap<&str, &str>) -> bool {
    let mut valid_fields = HashSet::new();
    valid_fields.insert("byr");
    valid_fields.insert("iyr");
    valid_fields.insert("eyr");
    valid_fields.insert("hgt");
    valid_fields.insert("hcl");
    valid_fields.insert("ecl");
    valid_fields.insert("pid");

    // Consider only keys.
    let mut cpy: HashSet<&str> = hs.iter().map(|(k, v)| *k).collect();
    // Make copy and remove optional field.
    cpy.remove("cid");
    valid_fields == cpy
}

fn is_valid_entry_part2(hs: &HashMap<&str, &str>) -> bool {
    // Must still be valid by first rule set:
    if ! is_valid_entry_part1(hs) {
        return false;
    }

    do_valid(hs).is_ok()
}

fn do_valid(hs: &HashMap<&str, &str>) -> anyhow::Result<()> {
    // dbg!(hs);
    let year = hs.get("byr").unwrap().parse::<u32>()?;
    ensure!(year >= 1920 && year <= 2002, "invalid year");

    let year = hs.get("iyr").unwrap().parse::<u32>()?;
    ensure!(year >= 2010 && year <= 2020, "invalid year");

    let year = hs.get("eyr").unwrap().parse::<u32>()?;
    ensure!(year >= 2020 && year <= 2030, "invalid year");

    let height = hs.get("hgt").unwrap();
    let iter = height.clone().chars();
    let num = iter.take_while(|c| c.is_numeric()).collect::<String>().parse::<u32>()?;
    let unit = height.chars().skip_while(|c| c.is_numeric()).collect::<String>();
    match unit.as_str() {
        "cm" => ensure!(num >= 150 && num <= 193, "cm height invalid"),
        "in" => ensure!(num >= 59 && num <= 76, "cm height invalid"),
        _ => bail!("No cm/in found in height."),
    }

    let hair_color = hs.get("hcl").unwrap();
    let hc: Vec<char> = hair_color.chars().collect();
    ensure!(hc.len() == 7, "Invalid hair color format");
    ensure!(hc[0] == '#', "Invalid hair color format");
    for c in & hc[1..] {
        ensure!(c.is_alphanumeric(), "Invalid hair color format");
    }

    let ec = hs.get("ecl").unwrap();
    let valids = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    ensure!(valids.contains(ec), "invalid eye color");

    let pid = hs.get("pid").unwrap();
    let p: Vec<char> = pid.chars().collect();
    ensure!(p.len() == 9, "Wrong passport id length");
    for c in p.iter() {
        ensure!(c.is_numeric(), "Invalid password id format");
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{parse_input, is_valid_entry_part1, is_valid_entry_part2};

    #[test]
    fn test1() {
        let input = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let c = parse_input(&input).iter().filter(|e| is_valid_entry_part1(&e)).count();
        assert_eq!(2, c);
    }

    #[test]
    fn test2() {
        let input = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";
        let c = parse_input(&input).iter().filter(|e| is_valid_entry_part2(&e)).count();
        assert_eq!(0, c);
    }

    #[test]
    fn test3() {
        let input = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

";
        let c = parse_input(&input).iter().filter(|e| is_valid_entry_part2(&e)).count();
        assert_eq!(4, c);
    }
}