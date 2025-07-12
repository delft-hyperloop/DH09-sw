use crate::dataflow::procedures::make_procedures;
use crate::dataflow::*;

pub fn make_gs_code(df: &DataflowSpec) -> String {
    let mut code = String::new();

    code.push_str(
        r#"
pub fn process_input_datatype(datatype: Datatype, data: u64) -> f64 {
"#,
    );
    code.push_str(&make_procedures(df));
    code.push_str("match datatype {");

    for dp in df.message_processing.iter().flat_map(|p| p.datapoint_conversion.iter()) {
        let x = format!("parse_{}", dp.gs.conversion.procedure_suffix);
        let dtn = &dp.datapoint.name;
        writeln!(&mut code, "Datatype::{dtn} => {x}(data),").unwrap();
    }
    writeln!(&mut code, "_ => data as f64,}}}}").unwrap();

    code
}
