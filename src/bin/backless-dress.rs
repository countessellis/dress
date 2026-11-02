use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use ::backless_dress::init;
use ::backless_dress::logger;

///////////// Main

fn main() {
  logger::init();
  init::run();
}
