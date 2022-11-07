fn main() {
    let smt_file = "/sys/devices/system/cpu/smt/control";
    let args: Vec<String> = std::env::args().collect();

    let smt_status = match args.get(1) {
        Some(val) => val.clone(),
        None => {
            // Take no action if no status is provided.
            std::process::exit(0);
        }
    };

    match smt_status.as_str() {
        "on" | "off" => {
            std::fs::write(smt_file, smt_status).expect("Failed to write to CPU SMT file");
        }
        _ => {
            println!("Unknown value '{}'", smt_status);
        }
    };
}
