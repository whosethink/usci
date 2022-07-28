mod usci;

use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use clap::Parser;
use anyhow::{Context, Result};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
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
      let _ = writeln!(std::io::stdout(), "Error: {}", err);
      exit(1);
    }
  }
}

fn generate_command(command: GenerateCommand) -> Result<()> {
  let mut writer: BufWriter<Box<dyn Write>> = match command.file {
    Some(ref file_path) => {
      BufWriter::new(Box::new(std::fs::File::create(file_path)?))
    }
    None => {
      BufWriter::new(Box::new(std::io::stdout()))
    }
  };
  let mut rng = rand::thread_rng();
  for _ in 0..command.count {
    let code = UsciCode::from_random(&mut rng);
    writeln!(writer, "{}", code.get_code())?;
  }
  Ok(())
}

fn verify_command(command: VerifyCommand) -> Result<()> {

  fn verify_code(code_str: &str, stdout: &mut StandardStream, color_spec: &mut ColorSpec) -> Result<()> {
    match UsciCode::from_str(code_str) {
      Ok(_code) => {
        stdout.set_color(color_spec.set_fg(Some(Color::Green)))?;
        writeln!(stdout, "TRUE  {}", code_str)?;
      }
      Err(_err) => {
        stdout.set_color(color_spec.set_fg(Some(Color::Red)))?;
        writeln!(stdout, "FALSE {}", code_str)?;
      }
    }
    Ok(())
  }

  let mut stdout = StandardStream::stdout(ColorChoice::Auto);
  let mut color_spec = ColorSpec::new();
  for code_str in command.codes.iter() {
    verify_code(code_str, &mut stdout, &mut color_spec)?;
  }
  if let Some(file_path) = command.file {
    let file = std::fs::File::open(file_path)
      .with_context(|| format!("Can not read file"))?;
    let mut code_lines = BufReader::new(file).lines();
    while let Some(code_result) = code_lines.next() {
      if let Ok(code_line) = code_result {
        for code_str in code_line.split(|c| char::is_whitespace(c) || c == ',') {
          if !code_str.is_empty() {
            verify_code(code_str, &mut stdout, &mut color_spec)?;
          }
        }
      }
    }
  }
  Ok(())
}

fn info_command(command: InfoCommand) -> Result<()> {
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

  #[clap(long = "file", short = 'f')]
  file: Option<PathBuf>,

  #[clap(long = "count", short = 'c', default_value = "1")]
  count: u32,

}

#[derive(Debug, Parser)]
struct VerifyCommand {

  #[clap(long = "file", short = 'f')]
  file: Option<PathBuf>,

  codes: Vec<String>
}

#[derive(Debug, Parser)]
struct InfoCommand {

  codes: Vec<String>

}