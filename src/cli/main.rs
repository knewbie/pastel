use std::io::{self, Write};

use atty::Stream;

mod cli;
mod colorpicker;
mod colorspace;
mod commands;
mod config;
mod error;
mod hdcanvas;
mod named;
mod parser;
mod utility;

use commands::Command;
use config::Config;
use error::{PastelError, Result};

use pastel::ansi::{self, Brush};
use pastel::Color;

type ExitCode = i32;

fn write_stderr(c: Color, title: &str, message: &str) {
    writeln!(
        io::stderr(),
        "{}: {}",
        Brush::from_environment(Stream::Stdout).paint(format!("[{}]", title), c),
        message
    )
    .ok();
}

fn run() -> Result<ExitCode> {
    let app = cli::build_cli();
    let global_matches = app.get_matches();

    let interactive_mode = atty::is(Stream::Stdout);

    if interactive_mode {
        output_vt100::init();
    }

    let color_mode = match global_matches
        .value_of("color-mode")
        .expect("required argument")
    {
        "24bit" => Some(ansi::Mode::TrueColor),
        "8bit" => Some(ansi::Mode::Ansi8Bit),
        "off" => None,
        "auto" => {
            if interactive_mode {
                let env_color_mode = std::env::var("PASTEL_COLOR_MODE").ok();
                match env_color_mode.as_ref().map(|s| s.as_str()) {
                    Some("8bit") => Some(ansi::Mode::Ansi8Bit),
                    Some("24bit") => Some(ansi::Mode::TrueColor),
                    Some("off") => None,
                    Some(value) => {
                        return Err(PastelError::UnknownColorMode(value.into()));
                    }
                    #[cfg(windows)]
                    None => {
                        // Assume 24bit support on Windows
                        Some(ansi::Mode::TrueColor)
                    }
                    #[cfg(not(windows))]
                    None => {
                        let env_colorterm = std::env::var("COLORTERM").ok();
                        match env_colorterm.as_ref().map(|s| s.as_str()) {
                            Some("truecolor") | Some("24bit") => Some(ansi::Mode::TrueColor),
                            _ => {
                                if global_matches.subcommand_name() != Some("paint")
                                    && global_matches.subcommand_name() != Some("colorcheck")
                                {
                                    write_stderr(Color::yellow(), "pastel warning",
                                    "Your terminal emulator does not appear to support 24-bit colors \
                                    (this means that the COLORTERM environment variable is not set to \
                                    'truecolor' or '24bit'). \
                                    pastel will fall back to 8-bit colors, but you will only be able \
                                    to see rough approximations of the real colors.\n\n\
                                    To fix this, follow these steps:\n  \
                                      1. Run 'pastel colorcheck' to test if your terminal\n     \
                                         emulator does support 24-bit colors. If this is the\n     \
                                         case, set 'PASTEL_COLOR_MODE=24bit' to force 24-bit\n     \
                                         mode and to remove this warning. Alternatively, make\n     \
                                         sure that COLORTERM is properly set by your terminal\n     \
                                         emulator.\n  \
                                      2. If your terminal emulator does not support 24-bit\n     \
                                         colors, set 'PASTEL_COLOR_MODE=8bit' to remove this\n     \
                                         warning or try a different terminal emulator.\n\n\
                                    \
                                    For more information, see https://gist.github.com/XVilka/8346728\n");
                                }
                                Some(ansi::Mode::Ansi8Bit)
                            }
                        }
                    }
                }
            } else {
                None
            }
        }
        _ => unreachable!("Unknown --color-mode argument"),
    };

    let config = Config {
        padding: 2,
        colorpicker_width: 48,
        colorcheck_width: 8,
        interactive_mode,
        brush: Brush::from_mode(color_mode),
    };

    if let (subcommand, Some(matches)) = global_matches.subcommand() {
        let command = Command::from_string(subcommand);
        command.execute(matches, &config)?;
    } else {
        unreachable!("Subcommand is required");
    }

    Ok(0)
}

fn main() {
    let result = run();
    match result {
        Err(PastelError::StdoutClosed) => {}
        Err(err) => {
            write_stderr(Color::red(), "pastel error", &err.message());
            std::process::exit(1);
        }
        Ok(exit_code) => {
            std::process::exit(exit_code);
        }
    }
}
