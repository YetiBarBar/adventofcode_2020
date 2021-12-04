use std::path::PathBuf;

fn transform_to_items(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|s| {
            s.split_whitespace()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn main() {
    // Read file to a single string
    let mut filepath: PathBuf = std::env::current_dir().unwrap();
    filepath.push("data");
    filepath.push("day_2020_4.data");
    let input_data = std::fs::read_to_string(filepath).unwrap();

    // Produce a Vec<Vec<item as String>>
    println!("{:?}", part_1(&transform_to_items(&input_data)));
    println!("{:?}", part_2(&transform_to_items(&input_data)));
}

impl TryFrom<Vec<String>> for Passport {
    type Error = ();

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut pass_builder = PassportBuilder::default();
        for item in &value {
            let pos = item.find(':').unwrap();
            let value = item[pos + 1..].to_string();
            let id = &item[0..pos].to_string();
            match id.as_str() {
                "byr" => {
                    pass_builder.byr(value);
                }
                "eyr" => {
                    pass_builder.eyr(value);
                }
                "iyr" => {
                    pass_builder.iyr(value);
                }
                "hgt" => {
                    pass_builder.hgt(value);
                }
                "hcl" => {
                    pass_builder.hcl(value);
                }
                "ecl" => {
                    pass_builder.ecl(value);
                }
                "pid" => {
                    pass_builder.pid(value);
                }
                "cid" => {
                    pass_builder.cid(value);
                }
                _ => (),
            }
        }
        pass_builder.try_build().ok_or(())
    }
}

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

#[derive(Default, Debug, Clone)]
struct PassportBuilder {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

macro_rules! setters {
    ($($p: ident),*) => {

        $(pub fn $p(&mut self, $p: String) {
            self.$p = Some($p);
        })*

};
    ($($p: ident,)*) => {

        $(pub fn $p(&mut self, $p: String) {
            self.$p = Some($p)
        })*

};
}

impl Passport {
    pub fn validate_byr(&self) -> bool {
        let byr = self.byr.parse::<usize>().unwrap_or_default();

        byr.ge(&1920) && byr.le(&2002)
    }

    pub fn validate_iyr(&self) -> bool {
        let iyr = self.iyr.parse::<usize>().unwrap_or_default();

        iyr.ge(&2010) && iyr.le(&2020)
    }

    pub fn validate_eyr(&self) -> bool {
        let eyr = self.eyr.parse::<usize>().unwrap_or_default();

        eyr.ge(&2020) && eyr.le(&2030)
    }

    pub fn validate_hgt(&self) -> bool {
        let (val1, val2) = self.hgt.split_at(self.hgt.len() - 2);
        let size = val1.parse::<isize>().unwrap_or_default();
        match val2 {
            "cm" => size.ge(&150) && size.le(&193),
            "in" => size.ge(&59) && size.le(&76),
            _ => false,
        }
    }

    pub fn validate_hcl(&self) -> bool {
        let charset = ('0'..='9').chain('a'..='f').collect::<Vec<_>>();

        self.hcl.len() == 7
            && self.hcl.starts_with('#')
            && self.hcl.chars().skip(1).all(|c| charset.contains(&c))
    }

    pub fn validate_ecl(&self) -> bool {
        let valids = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        valids.contains(&self.ecl.as_str())
    }

    pub fn validate_pid(&self) -> bool {
        self.pid.len() == 9 && self.pid.chars().all(|c| c.is_digit(10))
    }

    pub fn validate(&self) -> bool {
        self.validate_byr()
            && self.validate_eyr()
            && self.validate_hcl()
            && self.validate_ecl()
            && self.validate_hgt()
            && self.validate_iyr()
            && self.validate_pid()
    }
}

impl PassportBuilder {
    setters!(byr, iyr, eyr, hgt, hcl, ecl, pid, cid);

    pub fn try_build(self) -> Option<Passport> {
        let byr = self.byr?;
        let iyr = self.iyr?;
        let eyr = self.eyr?;
        let hgt = self.hgt?;
        let hcl = self.hcl?;
        let ecl = self.ecl?;
        let pid = self.pid?;

        Some(Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
        })
    }
}

#[must_use]
pub fn part_1(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .filter(|&v| Passport::try_from(v.clone()).is_ok())
        .count()
}

#[must_use]
pub fn part_2(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .filter_map(|v| Passport::try_from(v.clone()).ok())
        .filter(Passport::validate)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_part_1() {
        let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

        assert_eq!(part_1(&transform_to_items(input)), 2)
    }
    #[test]
    fn test_day4_part_2() {
        let valids = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;

        let invalids = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;

        assert_eq!(part_2(&transform_to_items(invalids)), 0);
        assert_eq!(part_2(&transform_to_items(valids)), 4);
    }
}
