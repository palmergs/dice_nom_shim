use magnus::{function, prelude::*, Error, Ruby};

use dice_nom::parsers::generator_parser;

fn roll(roll: String) -> String {
    let g = match generator_parser(roll.as_ref()) {
        Ok((_, g)) => g,
        Err(e) => return format!("{{\"error\": \"{}\"}}", e),
    };
    let mut rng = rand::thread_rng();
    let mut results_array = Vec::new();
    let n = 1;
    for _ in 0..n {
        results_array.push(g.generate(&mut rng));
    }
    let json = serde_json::to_string(&results_array);
    match json {
        Ok(json) => format!("{}", json),
        Err(e) => format!("{{\"error\": \"{}\"}}", e),
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("DiceNomShim")?;
    module.define_singleton_method("roll", function!(roll, 1))?;
    Ok(())
}
