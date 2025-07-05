use gslib::Datapoint;
use gslib::ProcessedData;
use gslib::ValueCheckResult;

/// Preprocessing data from the pod before sending to the frontend
pub fn process(datapoint: &Datapoint) -> ProcessedData {
    let style = match datapoint.datatype.check_bounds(datapoint.value) {
        ValueCheckResult::Fine => "".to_string(),
        ValueCheckResult::Warn => "yellow".to_string(),
        ValueCheckResult::Error => "warning".to_string(),
        ValueCheckResult::BrakeNow => "error".to_string(),
    };
    let value = gslib::process_input_datatype(datapoint.datatype, datapoint.value);

    let significant = (value * 1000.0).round() / 1000.0;

    ProcessedData {
        datatype: datapoint.datatype,
        value: significant,
        timestamp: datapoint.timestamp,
        style,
        units: datapoint.datatype.unit(),
    }
}
