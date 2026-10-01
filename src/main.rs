use {
  clap::CommandFactory,
  clap_complete::CompleteEnv,
  just::Arguments,
  std::{
    env,
    io::{self, Write},
    process,
  },
};

fn main() {
  if env::var_os("JUST_COMPLETE").is_some_and(|value| value == "bash")
    && env::args_os().nth(1).is_none()
  {
    write!(
      io::stdout(),
      "{}",
      include_str!("../etc/completion-registration-script.bash")
    )
    .ok();
    return;
  }

  CompleteEnv::with_factory(Arguments::command)
    .var("JUST_COMPLETE")
    .complete();

  if let Err(code) = just::run(env::args_os()) {
    process::exit(code);
  }
}
