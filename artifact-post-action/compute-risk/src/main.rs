/**
 * MIT License
 *
 * Copyright (c) 2024 Enalean
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use serde_json::{json, Value};
use std::error::Error;
use std::io::stdin;

fn main() -> Result<(), Box<dyn Error>> {
    let json: Value = serde_json::from_reader(stdin()).map_err(|e| {
        eprintln!("ser: {e}");
        e
    })?;

    let values = &json["current"]["values"].as_array();

    let field_severity_value = values
        .and_then(|fields| {
            fields
                .into_iter()
                .find(|&field| field["label"] == "Severity")
        })
        .and_then(|field| {
            field["values"]
                .as_array()
                .and_then(|values| values.first())
                .and_then(|value| value["label"].as_str())
                .and_then(|severity_str| severity_str.parse::<i64>().ok())
        });

    let field_probability_value = values
        .and_then(|fields| {
            fields
                .into_iter()
                .find(|&field| field["label"] == "Probability")
        })
        .and_then(|field| {
            field["values"]
                .as_array()
                .and_then(|values| values.first())
                .and_then(|value| value["label"].as_str()) // Extracting the "Probability" value as a string
                .and_then(|probability_str| probability_str.parse::<i64>().ok()) // Converting the string to an integer
        });

    // NEW CODE FOR DETECTABILITY
    let field_detectability_value = values
        .and_then(|fields| {
            fields
                .into_iter()
                .find(|&field| field["label"] == "Detectability")
        })
        .and_then(|field| {
            field["values"]
                .as_array()
                .and_then(|values| values.first())
                .and_then(|value| value["label"].as_str()) // Extracting the "Probability" value as a string
                .and_then(|detectability_str| detectability_str.parse::<i64>().ok()) // Converting the string to an integer
        });
    // END NEW CODE FOR DETECTABILITY

    let field_risk = json["tracker"]["fields"]
        .as_array()
        .and_then(|fields| {
            fields
                .iter()
                .find(|&field| field["label"] == "Risk")
        });

    if field_risk.is_none() {
        return Err("Cannot find field_risk")?;
    }

    let risk_values = field_risk.unwrap()["values"].as_array();

    if risk_values.is_none() {
        return Err("Cannot find Risk values")?;
    }

    if let (Some(severity_value), Some(probability_value), Some(detectability_value)) = (field_severity_value, field_probability_value, field_detectability_value) {
        let product = severity_value * probability_value * detectability_value;

        let matching_value = risk_values.unwrap().into_iter()
        .find(|&value| {
            let value_label = value["label"].as_str().unwrap_or_default();
            value_label == product.to_string()
        });

        if let Some(matching_value) = matching_value {
            let field_id = field_risk.unwrap()["field_id"].as_i64().unwrap_or(0);
            println!("{}", json!({
                "values": [{
                    "field_id": field_id,
                    "bind_value_ids": [
                        matching_value["id"]
                    ]
                }]
            }).to_string());

            Ok(())
        } else {
            return Err("Cannot find matching Risk value")?;
        }
    } else {
        return Err("Cannot find Severity or Probability field")?;
    }
}
