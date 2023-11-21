use crate::logging::{err, info};
use crate::parser::sections::types::VariableType;
use indexmap::IndexMap;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Default, Debug, Clone)]
pub struct SkelettVariableTuple {
    pub start: usize,
    pub end: usize,
    pub name: String,
}

/// this method is reponsible for getting variables from string
/// i.e '{{var_1}}{{var_2}}' -> ['var_1', 'var_2']
/// Nested behaviour is considered invalid
/// i.e '{{var_1{{var_2}}}}'
/// todo: shall be replaced by regex when look-ahead and look-behind are supported
pub fn get_variables_inside_string<'a>(
    input: &'a str,
) -> Result<Vec<SkelettVariableTuple>, String> {
    let mut variable_names = Vec::new();
    let mut stack: Vec<&str> = Vec::new();

    const START_CHAR: &str = "{{";
    const END_CHAR: &str = "}}";

    // minimum string length of have a variable is length of match characters * 2 + 1
    // example :
    // match_inside: {{}}, minimum input to have variable should be of length 5 i.e {{ a }}
    if input.len() < ((2 * 2) + 1) {
        return Ok(variable_names);
    }

    let mut back_idx: usize = 0;
    let mut front_idx: usize = 2;
    let mut variable_tuple_data = SkelettVariableTuple::default();

    loop {
        // When front index has passed the end of the string, break
        if front_idx == input.len() + 1 {
            break;
        }
        // Slice to compare with the match characters where variable is enclosed
        let slice = &input[back_idx..front_idx];

        if slice == START_CHAR {
            // invalid string variable
            if !stack.is_empty() {
                // return some error
                return Err(format!(
                    "[err] Parsing error happened at position {}",
                    front_idx
                ));
            }
            variable_tuple_data.start = back_idx;
            stack.push(slice);
        }

        if slice == END_CHAR {
            if !stack.is_empty() {
                variable_tuple_data.end = front_idx;
                variable_tuple_data.name = (&input
                    [variable_tuple_data.start + START_CHAR.len()..back_idx])
                    .trim()
                    .to_string();
                variable_names.push(variable_tuple_data.clone());
                stack.pop();
            }
        }

        front_idx += 1;
        back_idx += 1;
    }

    return Ok(variable_names);
}

fn replace_variable(template: &str, var_name: &str, replacement: &str) -> String {
    let pattern = r"\{\{\s*".to_string() + var_name + r"\s*\}\}";
    let regex = Regex::new(&pattern).expect("Invalid regex pattern");

    regex.replace_all(template, replacement).to_string()
}

fn get_prompt(msg: &str) -> String {
    print!("{}", info(msg));
    io::stdout().flush().unwrap();
    let mut prompt_str = String::new();
    let _prompt_result = io::stdin()
        .read_line(&mut prompt_str)
        .expect("Invalid input recieved!");
    // Removes enter i.e \n
    prompt_str.pop();
    prompt_str
}

pub fn variable_resolver(
    variables_map: &IndexMap<String, VariableType>,
    input_var: &VariableType,
    max_recursion: usize,
    cache: &mut HashMap<VariableType, String>,
) -> Result<String, String> {
    let mut input_var_string_copy = input_var.get_as_string()?;

    // If present in cache return directly
    match cache.get(input_var) {
        Some(val) => {
            return Ok(val.to_string());
        }
        None => {}
    }

    // Avoid infinite recursions
    if max_recursion == 0 {
        return Err(err(&format!(
            "Max recursion reached! looking for: {}",
            input_var_string_copy
        )));
    }

    // Get variables in string
    let vars_vec = get_variables_inside_string(&input_var_string_copy)?;

    // Base case, direct substitution :
    // my_var = 25
    // another_var = '{{my_var}}'
    match input_var {
        VariableType::String(_) => {
            if vars_vec.len() == 0 {
                cache.insert(input_var.clone(), input_var_string_copy.clone());
                return Ok(input_var_string_copy);
            }
        }
        VariableType::PromptVariable(_) => {
            if vars_vec.len() == 0 {
                let prompt_str = get_prompt(&input_var_string_copy);
                cache.insert(input_var.clone(), prompt_str.clone());
                return Ok(prompt_str);
            }
        }
    }

    // Nested case, recurse
    // my_var_a = 19
    // my_var_b =  '{{my_var_a}}'
    // another_var = '{{my_var_b}}'
    let recursion_num = max_recursion - 1;
    for var in vars_vec {
        let var_value = variables_map.get(&var.name);
        if var_value.is_none() {
            return Err(err(
                &format!("Couldn't resolve variable {}", var.name).to_string()
            ));
        }
        let var_value_resolved =
            variable_resolver(&variables_map, var_value.unwrap(), recursion_num, cache);

        input_var_string_copy = replace_variable(
            &input_var_string_copy,
            &var.name,
            &var_value_resolved.unwrap(),
        );
    }

    // Ask for input if variable is prompt
    match input_var {
        VariableType::PromptVariable(_) => {
            let prompt_str = get_prompt(&input_var_string_copy);
            return Ok(prompt_str);
        }
        VariableType::String(_) => {
            return Ok(input_var_string_copy);
        }
    }
}
