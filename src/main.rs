use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match arguments.as_slice() {
        [argument] if argument == "--stdio" => match quicksilver::server::run_stdio() {
            Ok(code) => ExitCode::from(code),
            Err(error) => {
                eprintln!("quicksilver: {error}");
                ExitCode::FAILURE
            }
        },
        [argument] if argument == "--version" || argument == "-V" => {
            println!(
                "quicksilver {} (source-only bootstrap)",
                env!("CARGO_PKG_VERSION")
            );
            ExitCode::SUCCESS
        }
        [argument] if argument == "--help" || argument == "-h" => {
            println!(
                "Quicksilver — Elixir language server\n\n\
                 Usage: quicksilver --stdio\n\n\
                 Options:\n  --stdio     Start the LSP server on standard input and output.\n  \
                 --version   Print the version.\n  --help      Print this help.\n\n\
                 Bootstrap coverage: document symbols and syntax diagnostics for open Elixir files.\n\
                 This version does not provide semantic diagnostics or execute project code."
            );
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!(
                "Use quicksilver --stdio to start the server. Use --help for available options."
            );
            ExitCode::from(2)
        }
    }
}
