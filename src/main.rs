mod usci;
mod common;

use std::io::{stdout, Write};
use std::process::exit;
use std::str::FromStr;
use clap::Parser;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::common::UsciResult;
use crate::usci::{Code01, Code02, Code03, Code04, Code05, UsciCode};

fn main() {
  let command: UsciCommand = UsciCommand::parse();
  let result = match command {
    UsciCommand::Generate(command) => {
      generate_command(command)
    }
    UsciCommand::Verify(command) => {
      verify_command(command)
    }
    UsciCommand::Info(command) => {
      info_command(command)
    }
  };
  match result {
    Ok(_) => {
      exit(0);
    },
    Err(err) => {
      let _ = writeln!(stdout(), "Error: {}", err);
      exit(1);
    }
  }
}

fn generate_command(command: GenerateCommand) -> UsciResult<()> {
  let mut rng = rand::thread_rng();
  for _ in 0..command.count {
    let code = UsciCode::from_random(&mut rng);
    writeln!(stdout(), "{}", code.get_code())?;
  }
  Ok(())
}

fn verify_command(command: VerifyCommand) -> UsciResult<()> {
  let mut stdout = StandardStream::stdout(ColorChoice::Auto);
  let mut color_spec = ColorSpec::new();
  for code_str in command.codes.iter() {
    let code_result = UsciCode::from_str(code_str);
    match code_result {
      Ok(_code) => {
        stdout.set_color(color_spec.set_fg(Some(Color::Green)))?;
        writeln!(stdout, "TRUE  {}", code_str)?;
      }
      Err(_err) => {
        stdout.set_color(color_spec.set_fg(Some(Color::Red)))?;
        writeln!(stdout, "FALSE {}", code_str)?;
      }
    }
  }
  Ok(())
}

fn info_command(command: InfoCommand) -> UsciResult<()> {
  let mut stdout = StandardStream::stdout(ColorChoice::Auto);
  let mut color_spec = ColorSpec::new();
  color_spec.set_bold(true);
  for code_str in command.codes.iter() {
    stdout.set_color(color_spec.set_fg(Some(Color::White)))?;
    writeln!(stdout, "统一社会信用代码: {}", code_str)?;
    let code_result = UsciCode::from_str(code_str);
    match code_result {
      Ok(code) => {
        stdout.set_color(color_spec.set_fg(Some(Color::Green)))?;
        writeln!(stdout, "{:>10}: {} {}", code.code01.get_code(), Code01::code_type(), code.code01.get_type())?;
        writeln!(stdout, "{:>10}: {} {}", code.code02.get_code(), Code02::code_type(),code.code02.get_type())?;
        writeln!(stdout, "{:>10}: {}", code.code03.to_string(), Code03::code_type())?;
        writeln!(stdout, "{:>10}: {}", code.code04.to_string(), Code04::code_type())?;
        writeln!(stdout, "{:>10}: {}", code.code05.to_string(), Code05::code_type())?;
      }
      Err(_err) => {
        stdout.set_color(color_spec.set_fg(Some(Color::Red)))?;
        writeln!(stdout, "FALSE {}", code_str)?;
      }
    }
  }
  Ok(())
}

#[derive(Debug, Parser)]
#[clap(about = "little tool of usci code")]
enum UsciCommand {

  #[clap(about = "Generate some usci codes")]
  Generate(GenerateCommand),

  #[clap(about = "Check if codes are valid")]
  Verify(VerifyCommand),

  #[clap(about = "Show the information of codes")]
  Info(InfoCommand)
}

#[derive(Debug, Parser)]
struct GenerateCommand {

  #[clap(long = "count", short = 'c', default_value = "1")]
  count: u32

}

#[derive(Debug, Parser)]
struct VerifyCommand {

  // #[clap(long = "file", short = 'f')]
  // file: Option<PathBuf>,

  codes: Vec<String>
}

#[derive(Debug, Parser)]
struct InfoCommand {

  codes: Vec<String>

}