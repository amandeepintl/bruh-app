use std::path::PathBuf;

fn print_usage() {
    println!("Usage:");
    println!("  bruh compile <input> [output]");
    println!("  bruh decode <input> [output]");
    println!("  bruh --help | bruh --version");
}

pub fn run(args: Vec<String>) -> Result<(), i32> {
    if args.len() < 2 {
        print_usage();
        return Err(1);
    }

    let is_quiet = args.iter().any(|a| a == "--quiet" || a == "-q");
    let _is_preview = args.iter().any(|a| a == "--preview" || a == "-p");

    let command = &args[1];
    if command == "--help" || command == "-h" || command == "help" {
        print_usage();
        return Ok(());
    }
    if command == "--version" || command == "version" {
        if !is_quiet {
            println!("bruh 0.1.0");
        }
        return Ok(());
    }
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
        "unregister" | "uninstall" => {
            if let Err(err) = bruh::uninstall() {
                eprintln!("{err}");
                return Err(1);
            }
            println!("uninstalled BRUH-BETTER successfully");
        }
        _ => {
            eprintln!("unknown command: {command}");
            return Err(1);
        }
    }

    Ok(())
}
