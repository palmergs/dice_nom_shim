use magnus::{function, prelude::*, Error, Ruby};
use std::collections::BTreeMap;
use std::i32::MAX;

use dice_nom::parsers::generator_parser;

fn roll(roll: String) -> String {
    let g = match generator_parser(roll.as_ref()) {
        Ok((_, g)) => g,
        Err(e) => return format!("{{\"error\": \"{}\"}}", e),
    };
    let mut rng = rand::rng();
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

#[derive(serde::Serialize)]
struct Histo {
    min: i32,
    max: i32,
    max_cnt: u32,
    map: BTreeMap<i32, u32>,
}

impl Histo {
    pub fn build(roll: String, count: u32) -> Histo {
        let mut histo = Histo {
            min: MAX,
            max: 0,
            max_cnt: 0,
            map: BTreeMap::new(),
        };
        let g = match generator_parser(roll.as_ref()) {
            Ok((_, g)) => g,
            Err(_) => return histo,
        };
        let mut rng = rand::rng();
        for _ in 0..count {
            let v = g.generate(&mut rng).sum();
            if v < histo.min {
                histo.min = v;
            }
            if v > histo.max {
                histo.max = v;
            }
            match histo.map.get(&v) {
                Some(n) => {
                    let cnt = n + 1;
                    if cnt > histo.max_cnt {
                        histo.max_cnt = cnt;
                    }
                    histo.map.insert(v, cnt);
                }
                None => {
                    histo.map.insert(v, 1);
                }
            }
        }
        histo
    }
}

fn histo(roll: String) -> String {
    let num = 5000.0;
    let histo = Histo::build(roll, num as u32);
    let mut chart_data = Vec::new();
    let mut total = 100.0;
    for k in histo.min..=histo.max {
        let count = histo.map.get(&k).unwrap_or(&0);
        let percentage = (*count as f64 / num) * 100.0;
        chart_data.push(serde_json::json!({
          "value": k,
          "count": count,
          "percentage": percentage,
          "total": total,
        }));
        total = total - percentage;
    }

    let json = serde_json::to_string(&chart_data);
    match json {
        Ok(json) => format!("{}", json),
        Err(err) => format!("{{\"error\": \"{}\"}}", err),
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("DiceNomShim")?;
    module.define_singleton_method("roll", function!(roll, 1))?;
    module.define_singleton_method("histo", function!(histo, 1))?;
    Ok(())
}
