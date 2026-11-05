use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use ::dress::init;
use ::dress::logger;

///////////// Main

fn main() {
  logger::init();
  init::run();
}
