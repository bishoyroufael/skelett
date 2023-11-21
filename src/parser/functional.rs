use rhai::{Dynamic, Engine};

/// This method shall be used to evaluate an expression inside of a string
/// and returns the value to the caller to be assigned to the variable.
/// It shall be used after variable substitution takes place.
fn eval_rhai_string_expression(expression_string: &str) -> Result<Dynamic, String> {
    // Init rhai engine
    let engine = Engine::new();
    let result: Dynamic = engine
        .eval_expression(expression_string)
        .map_err(|e| e.to_string())?;

    println!("{}", result);

    return Ok(result);
}
