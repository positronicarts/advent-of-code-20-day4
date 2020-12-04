use clap::Clap;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

#[macro_use]
extern crate lazy_static;
use regex::Regex;

#[derive(Clap)]
struct Opts {
    part: i32,
    input: String,
}

// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
// byr:1937 iyr:2017 cid:147 hgt:183cm
#[derive(Default, Debug)]
struct PassportEntry {
    ecl: Option<String>,
    eyr: Option<String>,
    pid: Option<String>,
    hcl: Option<String>,
    byr: Option<String>,
    iyr: Option<String>,
    cid: Option<String>,
    hgt: Option<String>,
}

impl FromStr for PassportEntry {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\S+):(\S+)").unwrap();
        }
        let mut pe = PassportEntry::default();
        let blocks = s.split_whitespace();
        for block in blocks {
            let cap = RE.captures_iter(block).next().unwrap();
            match &cap[1] {
                "ecl" => pe.ecl = Some(cap[2].to_string()),
                "eyr" => pe.eyr = Some(cap[2].to_string()),
                "pid" => pe.pid = Some(cap[2].to_string()),
                "hcl" => pe.hcl = Some(cap[2].to_string()),
                "byr" => pe.byr = Some(cap[2].to_string()),
                "iyr" => pe.iyr = Some(cap[2].to_string()),
                "cid" => pe.cid = Some(cap[2].to_string()),
                "hgt" => pe.hgt = Some(cap[2].to_string()),
                _ => panic!("Invalid key {}", &cap[1]),
            }
        }

        Ok(pe)
    }
}

impl PassportEntry {
    fn is_valid_pt1(&self) -> bool {
        self.ecl.is_some()
            && self.eyr.is_some()
            && self.pid.is_some()
            && self.hcl.is_some()
            && self.byr.is_some()
            && self.iyr.is_some()
            && self.hgt.is_some()
    }

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    // cid (Country ID) - ignored, missing or not.
    fn is_valid_pt2(&self) -> bool {
        let byr_valid = {
            match &self.byr {
                Some(byr) => match byr.parse::<u32>() {
                    Ok(i) => (i >= 1920) && (i <= 2002),
                    _ => false,
                },
                _ => false,
            }
        };

        let iyr_valid = {
            match &self.iyr {
                Some(iyr) => match iyr.parse::<u32>() {
                    Ok(i) => (i >= 2010) && (i <= 2020),
                    _ => false,
                },
                _ => false,
            }
        };

        let eyr_valid = {
            match &self.eyr {
                Some(eyr) => match eyr.parse::<u32>() {
                    Ok(i) => (i >= 2020) && (i <= 2030),
                    _ => false,
                },
                _ => false,
            }
        };

        let hgt_valid = match &self.hgt {
            Some(hgt) => {
                lazy_static! {
                    static ref CM_RE: Regex = Regex::new(r"^(\d+)cm$").unwrap();
                    static ref IN_RE: Regex = Regex::new(r"^(\d+)in$").unwrap();
                }
                let mut cm_cap = CM_RE.captures_iter(hgt);

                match cm_cap.next() {
                    Some(cm) => match cm[1].parse::<u32>() {
                        Ok(cm) => (cm >= 150) && (cm <= 193),
                        _ => false,
                    },
                    _ => {
                        let mut in_cap = IN_RE.captures_iter(hgt);
                        match in_cap.next() {
                            Some(inch) => match inch[1].parse::<u32>() {
                                Ok(inch) => (inch >= 59) && (inch <= 76),
                                _ => false,
                            },
                            _ => false,
                        }
                    }
                }
            }
            _ => false,
        };

        let hcl_valid = {
            lazy_static! {
                static ref HCL_RE: Regex = Regex::new(r"^#[a-f|0-9]{6}$").unwrap();
            }
            match &self.hcl {
                Some(hcl) => HCL_RE.is_match(hcl),
                _ => false,
            }
        };

        lazy_static! {
            static ref ECL_VALUES: [String; 7] = [
                "amb".to_string(),
                "blu".to_string(),
                "brn".to_string(),
                "gry".to_string(),
                "grn".to_string(),
                "hzl".to_string(),
                "oth".to_string()
            ];
        }

        let ecl_valid = {
            match &self.ecl {
                Some(ecl) => ECL_VALUES.contains(&ecl),
                _ => false,
            }
        };

        let pid_valid = {
            lazy_static! {
                static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
            }
            match &self.pid {
                Some(pid) => PID_RE.is_match(pid),
                _ => false,
            }
        };

        byr_valid && iyr_valid && eyr_valid && hgt_valid && hcl_valid && ecl_valid && pid_valid
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    let pes = get_pes(opts.input);
    if opts.part == 1 {
        let valid_count = pes.iter().filter(|x| x.is_valid_pt1()).count();
        println!("There were {} valid passports", valid_count);
    } else {
        let valid_count = pes.iter().filter(|x| x.is_valid_pt2()).count();
        println!("There were {} valid passports", valid_count);
    }
}

fn get_pes(filename: String) -> Vec<PassportEntry> {
    let mut pes = Vec::<PassportEntry>::new();

    let mut current_line = "".to_string();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_as_string) = line {
                if line_as_string == "" {
                    pes.push(PassportEntry::from_str(&current_line).unwrap());
                    current_line = "".to_string();
                } else {
                    current_line += &" ";
                    current_line += &line_as_string;
                };
            }
        }
    }

    if current_line != "" {
        pes.push(PassportEntry::from_str(&current_line).unwrap());
    }

    pes
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
