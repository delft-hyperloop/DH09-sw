use gslib::{Datapoint, Limit};
use gslib::ProcessedData;
use gslib::ValueCheckResult;

/// Preprocessing data from the pod before sending to the frontend
pub fn process(datapoint: &Datapoint) -> ProcessedData {
    let value = gslib::process_input_datatype(datapoint.datatype, datapoint.value);
    let significant = (value * 1000.0).round() / 1000.0;

    let style = match datapoint.datatype.check_bounds(significant as u64) {
        ValueCheckResult::Fine => "".to_string(),
        ValueCheckResult::Warn => "text-yellow-400".to_string(),
        ValueCheckResult::Error => "text-warning-400".to_string(),
        ValueCheckResult::BrakeNow => "text-error-400".to_string(),
    };

    let bounds = datapoint.datatype.bounds();
    let mut upper = None;
    let mut lower = None;
    match (bounds.0, bounds.1) {
        (Limit::No, Limit::No) => {}
        (Limit::Single(upper_limit), Limit::Single(lower_limit)) => { 
            upper = Some(upper_limit);
            lower = Some(lower_limit);
        },
        (Limit::Multiple(upper_severities), Limit::Multiple(lower_severities)) => { 
            if upper_severities.brake.is_some() && lower_severities.brake.is_some() {
                upper = Some(upper_severities.brake.unwrap());
                lower = Some(lower_severities.brake.unwrap());
            }
        },
        _ => {}
    }

    ProcessedData {
        datatype: datapoint.datatype,
        value: significant,
        timestamp: datapoint.timestamp,
        style,
        units: datapoint.datatype.unit(),
        lower,
        upper,
    }
}
