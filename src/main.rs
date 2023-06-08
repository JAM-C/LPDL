use std::vec::Vec;
use std::fs;

enum Type {
    MAX,
    MIN
}

#[derive(Debug, PartialEq, Eq)]
enum EquationType {
    COST,
    CONSTRAINS
}

struct LPP {
    p_type: Type,
    var_idx: Vec<String>,
    obj_fun: Vec<i32>,
    constrains: Vec<Vec<i32>>,
    int: Vec<String>,
}

fn load(filename: String) -> String {
    match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(e) => {
            println!("{}", e);
            "".to_string()
        }
    }
}

fn only_once(lst: Vec<bool>) -> bool {
    let mut once = false;
    for b in &lst {
        if !once && *b {
            once = true;
        } else if once && *b {
            return false;
        }
    }

    return once;
}

fn remove_whitespace(str: &str) -> String {
    str.chars().filter(|c| !c.is_whitespace()).collect()
}

fn comparison_condition_once(p_obj_str: &str) -> bool {
    let comps = ["<","<=", ">", ">=", "="];
    only_once(comps
                .map(|c| p_obj_str.contains(c)).to_vec())
}

fn parse_equation(var_names: &mut Vec<String>, obj_coefficients: &mut Vec<f32>, p_obj_str: &str, equ_type: EquationType) {
    let mut tracked_coefficient: String = "".to_string();
    let mut tracked_variable: String = "".to_string();
    let mut on_variable: bool = false;

    for c in p_obj_str.chars() {

        if c == '*' {
            continue;
        }

        if ['+', '-', '<', '>', '='].contains(&c) && on_variable {
            let mut a = tracked_coefficient.parse::<f32>().unwrap_or(1.0);
            if c == '-' {
                a = -a;
            }

            if equ_type == EquationType::COST {
                var_names.push(tracked_variable);
                obj_coefficients.push(a);
            } else {
                obj_coefficients[var_names.iter().position(|c| *c == tracked_variable).unwrap()] = a;
            }

            on_variable = false;
            tracked_coefficient = "".to_string();
            tracked_variable = "".to_string();
            continue;
            
        } else if ['+', '-', '<', '>', '='].contains(&c) {
            continue;
        }

        if (c.is_digit(10) || c == '.') && !on_variable {
            tracked_coefficient += &c.to_string();
        } else if c != '.' {
            on_variable = true;
            tracked_variable += &c.to_string();
        }
    }

    println!("DDD {}", tracked_coefficient.parse::<f32>().unwrap());
    obj_coefficients.push(tracked_coefficient.parse::<f32>().unwrap());

}

fn parse(data: String) -> Result<LPP, &'static str> {
    let problem: LPP;

    let lines: Vec<String> = data.split("\n").map(remove_whitespace).filter(|c| c.len() > 0).collect();

    // get type
    let p_first_line_str = lines[0].split(":").collect::<Vec<&str>>();

    if p_first_line_str.len() != 2 {
        return Err("Error")
    }

    if p_first_line_str[0] != "MAX" && p_first_line_str[0] != "MIN" {
        return Err("Error")
    }

    let p_type = if p_first_line_str[0] == "MAX" {Type::MAX} else {Type::MIN}; 

    // get objective function
    let p_obj_str = p_first_line_str[1];

    let mut var_names: Vec<String> = Vec::new();
    let mut obj_coefficients: Vec<f32> = Vec::new();

    if !comparison_condition_once(p_obj_str) {
        return Err("Error");
    }

    parse_equation(&mut var_names, &mut obj_coefficients, &p_obj_str, EquationType::COST);

    println!("{:?}", var_names);
    println!("{:?}", obj_coefficients);

    // get constrains

    let mut constrains: Vec<Vec<f32>> = vec![vec![0.0; var_names.len() + 1]; lines.len()-1];

    for (i, s) in lines[1..].iter().enumerate() {
        parse_equation(&mut var_names, &mut constrains[i], s.as_str(), EquationType::CONSTRAINS);
        // TODO: Adjust for int constrains
    }

    println!("{:?}", constrains);

    Err("problem")
}

fn main() {
    parse(load("example.txt".to_string()));
}

