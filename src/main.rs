use crate::common::*;

mod common;
mod display_command;
mod error;
mod result;
mod splitter;

fn run() -> Result<()> {
  const CARGO: &str = "cargo";

  env_logger::init();

  let arguments = env::args_os().skip(2).collect::<Vec<OsString>>();

  let arguments = if arguments.is_empty() {
    vec!["check".into()]
  } else {
    arguments
  };

  info!(
    "Executing command: {}",
    DisplayCommand::new(OsStr::new(CARGO), &arguments)
  );

  let mut command = Command::new(CARGO);

  command
    .args(&arguments)
    .env("CARGO_TERM_COLOR", "always")
    .stderr(Stdio::piped());

  if let Some((width, _height)) = term_size::dimensions() {
    command
      .env("CARGO_TERM_PROGRESS_WHEN", "always")
      .env("CARGO_TERM_PROGRESS_WIDTH", &width.to_string());
  }

  let mut child = command.spawn().with_context(|| error::Spawn {
    arguments: arguments.clone(),
    command:   CARGO,
  })?;

  let errorfile = File::create(".errors.txt").context(error::Filesystem {
    path: ".errors.txt",
  })?;

  let writer = strip_ansi_escapes::Writer::new(errorfile);

  let mut splitter = Splitter::new(writer, io::stderr());

  let stderr = child.stderr.as_mut().unwrap();

  io::copy(stderr, &mut splitter).context(error::Copy {
    path: ".errors.txt",
  })?;

  let status = child.wait().with_context(|| error::Wait {
    arguments: arguments.clone(),
    command:   CARGO,
  })?;

  if !status.success() {
    return Err(Error::Status {
      command: CARGO.into(),
      status,
      arguments,
    });
  }

  Ok(())
}

fn main() {
  if let Err(error) = run() {
    if error.display() {
      eprintln!("error: {}", error);
    }
    process::exit(error.code());
  }
}
