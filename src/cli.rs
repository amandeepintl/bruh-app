use std::path::PathBuf;

pub fn run(args: Vec<String>) -> Result<(), i32> {
    if args.len() < 2 {
        eprintln!("usage: bruh <compile|decode> <input> [output]");
        return Err(1);
    }

    let command = &args[1];
    let input = PathBuf::from(args.get(2).unwrap_or(&String::new()));
    let output = args.get(3).map(PathBuf::from).unwrap_or_else(|| {
        if command == "compile" {
            input.with_extension("bruh")
        } else {
            input.with_extension("png")
        }
    });

    match command.as_str() {
        "compile" => {
            if input.as_os_str().is_empty() {
                eprintln!("missing input path");
                return Err(1);
            }
            if let Err(err) = bruh::compile_image(&input, &output) {
                eprintln!("{err}");
                return Err(1);
            }
            println!("compiled {} -> {}", input.display(), output.display());
        }
        "decode" => {
            if input.as_os_str().is_empty() {
                eprintln!("missing input path");
                return Err(1);
            }
            if let Err(err) = bruh::decode_image(&input, &output) {
                eprintln!("{err}");
                return Err(1);
            }
            println!("decoded {} -> {}", input.display(), output.display());
        }
        _ => {
            eprintln!("unknown command: {command}");
            return Err(1);
        }
    }

    Ok(())
}
