#[cfg(target_os = "macos")]
fn main() {}

#[cfg(not(target_os = "macos"))]
fn main() -> Result<(), rppal::spi::Error> {
  use rppal::spi::{Spi, Bus, SlaveSelect, Mode};
  use rand::Rng;

  use p9813::P9813;

  let spi = Spi::new(
    Bus::Spi0,
    SlaveSelect::Ss0,
    P9813::MAX_CLOCK_FREQUENCY,
    Mode::Mode0,
  )?;

  let mut p9813 = P9813::new(spi);

  let r = rand::thread_rng().gen();
  let g = rand::thread_rng().gen();
  let b = rand::thread_rng().gen();

  println!("Setting color to ({}, {}, {}).", r, g, b);
  p9813.set_color(r, g, b)
}
