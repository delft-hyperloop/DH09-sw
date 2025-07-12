use crate::dataflow::*;

pub fn make_procedures(df: &DataflowSpec) -> String {
    let mut code = String::from("");

    for (name, spec) in &df.procedures {
        code.push_str(&format!(
            "fn {}(x: {}) -> {} {{\n",
            name.replace('-', "_"),
            spec.input,
            spec.output,
        ));
        code.push_str(&spec.formula);
        code.push_str("}\n\n");
    }

    code
}
