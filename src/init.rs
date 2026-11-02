use crate::cli::*;
use crate::config::*;
use crate::help::*;
use crate::mode::*;
use crate::splash::*;
use crate::version::*;

///////////// Run

pub fn run() {
  println!("{}",splash());
  println!("{}",version());
  log::info!("{}",version());
  let config: Config = Config::new();
  log::info!("{}",config);
  let mode: Mode = Mode::mode();
  match mode {
    Mode::CLI => {
      let _cli: CLI = CLI::new(&config);
    },
    Mode::Help => {
      let help: Help = Help::new(&config);
      println!("{}",help);
    },
    _ => log::error!("Mode {} not implemented.",mode),
  }
}
