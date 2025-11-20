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

/// Extracts an i64 from the "current.values" array given the field label.
/// Looks for:
/// current.values[*].label == field_label
/// and then takes values[0].label and parses it as i64.
/// Returns None if any step fails.
fn get_current_numeric_value(json: &Value, field_label: &str) -> Option<i64> {

    json["current"]["values"].as_array()
        .and_then(|fields| {fields.into_iter().find(|&field| field["label"] == field_label)})
        .and_then(|field| {field["values"].as_array()
        .and_then(|values| values.first())
        .and_then(|value| value["label"].as_str()) // Extracting value as a string
        .and_then(|detectability_str| detectability_str.parse::<i64>().ok()) // Converting the string to an integer
        })
}

/// This function computes automacilly the risk level before and after mitigation based on
/// the severity, probability and detectability fields.
/// Limitations:
/// - the severity, probability and detectability fields must have numeric values
/// - the risk level field must have values corresponding to the product of severity * probability * detectability
/// If any of these conditions is not met, the function will return an error.
fn main() -> Result<(), Box<dyn Error>> {
    let json: Value = serde_json::from_reader(stdin()).map_err(|e| {
        eprintln!("ser: {e}");
        e
    })?;

    // Retrieve fields values before mitigations
    let field_severity_before_value = get_current_numeric_value(&json, "Severity before mitigation");
    let field_probability_before_value = get_current_numeric_value(&json, "Probability before mitigation");
    let field_detectability_before_value = get_current_numeric_value(&json, "Detectability before mitigation");

    // Check that the risk before mitigation field exists
    let field_risk = json["tracker"]["fields"].as_array()
        .and_then(|fields| {fields.iter().find(|&field| field["label"] == "Risk level before mitigation")});

    if field_risk.is_none() {
        return Err("Cannot find field_risk")?;
    }

    let risk_values = field_risk.unwrap()["values"].as_array();

    if risk_values.is_none() {
        return Err("Cannot find Risk values")?;
    }

    // Retrieve fields values after mitigations
    let field_severity_after_value = get_current_numeric_value(&json, "Severity after mitigation");
    let field_probability_after_value = get_current_numeric_value(&json, "Probability after mitigation");
    let field_detectability_after_value = get_current_numeric_value(&json, "Detectability after mitigation");

    // Check that the risk after mitigation field exists
    let field_risk_after = json["tracker"]["fields"].as_array()
        .and_then(|fields| {fields.iter().find(|&field| field["label"] == "Risk level after mitigation")});

    if field_risk_after.is_none() {
        return Err("Cannot find field_risk_after")?;
    }

    let risk_values_after = field_risk_after.unwrap()["values"].as_array();

    if risk_values_after.is_none() {
        return Err("Cannot find Risk values after mitigation")?;
    }

    // Check if all required values are present and compute risk levels
    if let (Some(severity_value_before),
            Some(probability_value_before),
            Some(detectability_value_before),
            Some(severity_value_after),
            Some(probability_value_after),
            Some(detectability_value_after)) = (field_severity_before_value, field_probability_before_value, field_detectability_before_value, field_severity_after_value, field_probability_after_value, field_detectability_after_value) {

        let product_before = severity_value_before * probability_value_before * detectability_value_before;
        let product_after = severity_value_after * probability_value_after * detectability_value_after;

        // Find matching risk values based on computed products
        let matching_value_before = risk_values.unwrap().into_iter()
        .find(|&value| {
            let value_label = value["label"].as_str().unwrap_or_default();
            value_label == product_before.to_string()
        });

        let matching_value_after = risk_values_after.unwrap().into_iter()
        .find(|&value| {
            let value_label = value["label"].as_str().unwrap_or_default();
            value_label == product_after.to_string()
        });

        // Update the risk level fields with matching values
        if let (Some(matching_value_before), Some(matching_value_after)) = (matching_value_before, matching_value_after) {
            let field_id_before = field_risk.unwrap()["field_id"].as_i64().unwrap_or(0);
            let field_id_after = field_risk_after.unwrap()["field_id"].as_i64().unwrap_or(0);
            println!("{}", json!({
                "values": [{
                    "field_id": field_id_before,
                    "bind_value_ids": [
                        matching_value_before["id"]
                    ]
                },
                {
                    "field_id": field_id_after,
                    "bind_value_ids": [
                        matching_value_after["id"]
                    ]
                }]
            }).to_string());

        } else {
            return Err("Cannot find matching Risk value")?;
        }
    } else {
        return Err("Cannot find Severity or Probability field")?;
    }

    Ok(())
}
