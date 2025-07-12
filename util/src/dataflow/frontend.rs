use crate::dataflow::*;

pub fn generate_types(df: &DataflowSpec) -> String {
    let mut code = String::new();
    write!(
        &mut code,
        r#"
/* BEGIN AUTO GENERATED TYPES */
export type NamedCommand = "#
    )
    .unwrap();

    for (id, command) in df.commands.iter().enumerate() {
        if id != 0 {
            write!(&mut code, " | ").unwrap();
        }
        write!(&mut code, "\"{}\"", command.name).unwrap();
    }

    writeln!(
        &mut code,
        r#";
export const NamedCommandValues:NamedCommand[] = ["#
    )
    .unwrap();
    for (id, command) in df.commands.iter().enumerate() {
        if id != 0 {
            write!(&mut code, ", ").unwrap();
        }
        write!(&mut code, "\"{}\"", command.name).unwrap();
    }
    write!(
        &mut code,
        r#"];

export type NamedDatatype = "#
    )
    .unwrap();

    let dt = collect_data_types(df);

    dt.Datatype.iter().enumerate().for_each(|(id, datatype)| {
        if id != 0 {
            write!(&mut code, " | ").unwrap();
        }
        write!(&mut code, "\"{}\"", datatype.name).unwrap();
    });

    writeln!(
        &mut code,
        r#";

export const NamedDatatypeValues = ["#
    )
    .unwrap();

    dt.Datatype.iter().enumerate().for_each(|(id, datatype)| {
        if id != 0 {
            write!(&mut code, ", ").unwrap();
        }
        write!(&mut code, "\"{}\"", datatype.name).unwrap();
    });

    writeln!(&mut code, r#"];"#).unwrap();

    code
}

pub fn generate_stores(df: &DataflowSpec) -> String {
    let mut code = String::new();
    let dt = collect_data_types(df);

    writeln!(
        &mut code,
        r#"
        // gdd stores registration
        // auto-generated with npm run generate:gs
        "#
    )
    .unwrap();

    for d in &dt.Datatype {
        if let Some(store) = &d.store {
            write!(
                &mut code,
                r#"
            gdd.stores.registerStore<{type}>("{name}", {default}"#,
                type = store.ty,
                name = d.name,
                default = store.default,
            )
            .unwrap();
            if let Some(callback) = &store.callback {
                write!(&mut code, ", {callback}").unwrap();
            }
            match (d.lower, d.upper) {
                (Limit::Single(lower), Limit::Single(upper)) => {
                    writeln!(&mut code, ", undefined, {lower}, {upper}").unwrap()
                },
                (Limit::Multiple(lower_severities), Limit::Multiple(upper_severities)) => {
                    if lower_severities.brake.is_some() && upper_severities.brake.is_some() {
                        writeln!(
                            &mut code,
                            ", undefined, {}, {}",
                            lower_severities.brake.unwrap(),
                            upper_severities.brake.unwrap()
                        )
                        .unwrap()
                    }
                },
                _ => {},
            }
            writeln!(&mut code, ");").unwrap();
        }
    }

    code
}
