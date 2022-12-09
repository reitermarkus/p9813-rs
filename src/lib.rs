#![no_std]
#![deny(bad_style, future_incompatible, missing_docs, missing_debug_implementations, rust_2018_idioms)]

//! A library for the P9813 RGB controller.

use core::fmt;

use embedded_hal::blocking::spi::Write;

const FLAG_BITS: u8 = 0b11_00_00_00;
const FRAME_START: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
const FRAME_END: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

/// Struct representing a P9813 controller.
pub struct P9813<S> {
  spi: S,
}

impl<S: fmt::Debug> fmt::Debug for P9813<S> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("P913").field("spi", &self.spi).finish()
  }
}

impl P9813<()> {
  /// Maximum frequency supported by the P9813.
  pub const MAX_CLOCK_FREQUENCY: u32 = 15_000_000;
}

impl<S: Write<u8>> P9813<S> {
  /// Create a new `P9813` instance with `spi`
  /// as the underlying SPI interface.
  pub fn new(spi: S) -> Self {
    P9813 { spi }
  }

  /// Set color for a single P9813.
  ///
  /// ```
  /// # fn f<S: embedded_hal::blocking::spi::Write<u8>>(spi: S) -> Result<(), S::Error> {
  /// # use p9813::P9813;
  /// let mut p9813 = P9813::new(spi);
  /// p9813.set_color(0, 255, 200)?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn set_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), S::Error> {
    self.set_colors([(r, g, b)])
  }

  /// Set colors for multiple P9813s chained together.
  ///
  /// ```
  /// # fn f<S: embedded_hal::blocking::spi::Write<u8>>(spi: S) -> Result<(), S::Error> {
  /// # use p9813::P9813;
  /// let mut p9813 = P9813::new(spi);
  /// p9813.set_colors([(0, 255, 200), (255, 50, 20)])?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn set_colors(&mut self, colors: impl AsRef<[(u8, u8, u8)]>) -> Result<(), S::Error> {
    self.spi.write(&FRAME_START)?;

    for &(r, g, b) in colors.as_ref().iter() {
      let b_bit = !(b >> 6) & 0b11;
      let g_bit = !(g >> 6) & 0b11;
      let r_bit = !(r >> 6) & 0b11;

      let prefix = FLAG_BITS | (b_bit << 4) | (g_bit << 2) | r_bit;

      self.spi.write(&[prefix, b, g, r])?;
    }

    self.spi.write(&FRAME_END)
  }
}
