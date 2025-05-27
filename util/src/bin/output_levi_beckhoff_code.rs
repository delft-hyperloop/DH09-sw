fn main() {
    let df_path = std::env::args().nth(1).expect("No dataflow file provided");
    let df = std::fs::read_to_string(df_path).expect("Failed to read dataflow file");
    let df = goose_utils::dataflow::parse_from(&df);

    let output = goose_utils::dataflow::make_levi_beckhoff_code(&df);
    println!("{}", output);
}