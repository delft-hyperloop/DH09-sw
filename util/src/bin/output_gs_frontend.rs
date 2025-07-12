use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let project_root =
        PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();

    let df_path = project_root.join("config/dataflow.yaml"); // std::env::args().nth(1).expect("No dataflow file provided");
    let ts_path = project_root.join("gs/src/lib/types.ts"); // std::env::args().nth(2).expect("No types.ts file provided");
    let stores_path = project_root.join("gs/src/routes/+layout.svelte");

    let ts_file = PathBuf::from(&ts_path).canonicalize().expect("could not find types.ts");
    let stores_file =
        PathBuf::from(&stores_path).canonicalize().expect("could not find +layout.svelte");

    let df = std::fs::read_to_string(df_path).expect("Failed to read dataflow file");
    let df = goose_utils::dataflow::parse_from(&df);

    let types = goose_utils::dataflow::frontend::generate_types(&df);

    let mut content = String::new();
    let mut f = File::open(&ts_file).expect("could not open types.ts");
    f.read_to_string(&mut content)?;

    let output = content
        .lines()
        .take_while(|line| !line.contains("BEGIN AUTO GENERATED TYPES"))
        .chain(types.lines())
        .chain(content.lines().skip_while(|line| !line.contains("END AUTO GENERATED TYPES")))
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    drop(ts_file);

    File::create(ts_path).unwrap().write_all(output.as_bytes()).unwrap();

    let stores = goose_utils::dataflow::frontend::generate_stores(&df);

    let mut content = String::new();
    let mut f = File::open(&stores_file).expect("could not open +layout.svelte");
    f.read_to_string(&mut content)?;

    let output = content
        .lines()
        .take_while(|line| !line.contains("BEGIN AUTO GENERATED STORES"))
        .chain(stores.lines())
        .chain(content.lines().skip_while(|line| !line.contains("END AUTO GENERATED STORES")))
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    drop(stores_file);

    File::create(stores_path).unwrap().write_all(output.as_bytes()).unwrap();

    Ok(())
}
