use std::vec::Vec;
use std::fs;
use std::env;

#[derive(Debug)]
enum Type {
    MAX,
    MIN
}

#[derive(Debug, PartialEq, Eq)]
enum EquationType {
    COST,
    CONSTRAINS
}

#[derive(Debug)]
struct LPP {
    p_type: Type,
    var_idx: Vec<String>,
    obj_fun: Vec<f32>,
    constrains: Vec<Vec<f32>>,
    int_constrains: Vec<String>,
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

fn remove_unused_data(str: &str) -> String {
    let line = str.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let comment_idx = line.find("//").unwrap_or(line.len());
    line[0..comment_idx].to_string()
}

fn comparison_condition_once(p_obj_str: &str) -> bool {
    ["<","<=", ">", ">=", "="]
        .map(|c| p_obj_str.contains(c))
        .iter()
        .filter(|c| **c)
        .count() == 1
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
            let value = if c == '-' {1.0} else {-1.0} * tracked_coefficient.parse::<f32>().unwrap_or(1.0);

            if equ_type == EquationType::COST {
                var_names.push(tracked_variable);
                obj_coefficients.push(value);
            } else {
                let var_idx = var_names.iter().position(|c| *c == tracked_variable);
                if var_idx.is_some() {
                    obj_coefficients[var_idx.unwrap()] = value;
                }
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

    obj_coefficients.push(tracked_coefficient.parse::<f32>().unwrap());
}

fn is_int_constrain(line: &str, obj_coefficients: &Vec<String>) -> bool {
    let constrain = line.split(":").collect::<Vec<&str>>();
    constrain.len() == 2 && obj_coefficients.contains(&constrain[0].to_string()) && constrain[1] == "int"
}

fn filter_unused(s: &String) -> bool {
    s.len() > 0
}

fn parse(data: String) -> Result<LPP, &'static str> {

    let lines: Vec<String> = data.split("\n").map(remove_unused_data).filter(filter_unused).collect();

    // get type
    let p_first_line_str = lines[0].split(":").collect::<Vec<&str>>();

    if p_first_line_str.len() != 2 {
        return Err("Error");
    }

    if p_first_line_str[0] != "MAX" && p_first_line_str[0] != "MIN" {
        return Err("Error");
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


    // get constrains

    let mut constrains: Vec<Vec<f32>> = vec![vec![0.0; var_names.len() + 1]; lines.len()-1];
    // adjust to reduce size when :int
    let mut int_constrains: Vec<String> = Vec::new();

    for (i, s) in lines[1..].iter().enumerate() {
        if is_int_constrain(s.as_str(), &var_names) {
            int_constrains.push(var_names[i].clone());
        } else  if comparison_condition_once(p_obj_str) {
            parse_equation(&mut var_names, &mut constrains[i], s.as_str(), EquationType::CONSTRAINS);
        }
    }

    let problem: LPP = {LPP {
        p_type: p_type,
        var_idx: var_names,
        obj_fun: obj_coefficients,
        constrains: constrains,
        int_constrains: int_constrains,
    }};
    
    println!("{:?}", problem);
    Ok(problem)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    parse(load(args[1].to_string()));
}
