#![no_std]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

//! A library for the P9813 RGB controller.

use embedded_hal::spi::SpiDevice;

const FLAG_BITS: u8 = 0b11_00_00_00;
const FRAME_START: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
const FRAME_END: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

//    ╭─────┬────────┬────────┬────────╮
//  0 │ 1 1 │ B7'B6' │ G7'G6' │ R7'R6' │
//    ├─────┴────────┴────────┴────────┤
//  8 │ B7  B6  B5  B4  B3  B2  B1  B0 │
//    ├────────────────────────────────┤
// 16 │ G7  G6  G5  G4  G3  G2  G1  G0 │
//    ├────────────────────────────────┤
// 24 │ R7  R6  R5  R4  R3  R2  R1  R0 │
//    ╰────────────────────────────────╯
fn color_to_array(r: u8, g: u8, b: u8) -> [u8; 4] {
  let b_bit = !b >> 6;
  let g_bit = !g >> 6;
  let r_bit = !r >> 6;

  let prefix = FLAG_BITS | (b_bit << 4) | (g_bit << 2) | r_bit;
  [prefix, b, g, r]
}

/// Struct representing a P9813 controller.
#[derive(Debug)]
pub struct P9813<SPI> {
  spi: SPI,
}

impl P9813<()> {
  /// Maximum frequency supported by the P9813.
  pub const MAX_CLOCK_FREQUENCY: u32 = 15_000_000;
}

impl<SPI: SpiDevice<u8>> P9813<SPI> {
  /// Create a new `P9813` with the given SPI peripheral.
  pub const fn new(spi: SPI) -> Self {
    P9813 { spi }
  }

  /// Release the contained SPI peripheral.
  pub fn release(self) -> SPI {
    self.spi
  }

  /// Set color for a single P9813.
  ///
  /// ```
  /// # fn main() -> Result<(), embedded_hal::spi::ErrorKind> {
  /// # use embedded_hal_mock::eh1::{spi::{Mock as SpiMock, Transaction as SpiTransaction}, delay::NoopDelay};
  /// # let spi = SpiMock::new(&[
  /// #   // Start frame.
  /// #   SpiTransaction::transaction_start(),
  /// #   SpiTransaction::write_vec(vec![0x00, 0x00, 0x00, 0x00]),
  /// #   SpiTransaction::transaction_end(),
  /// #
  /// #   // Set color.
  /// #   SpiTransaction::transaction_start(),
  /// #   SpiTransaction::write_vec(vec![0b11000011, 200, 255, 0]),
  /// #   SpiTransaction::transaction_end(),
  /// #
  /// #   // End frame.
  /// #   SpiTransaction::transaction_start(),
  /// #   SpiTransaction::write_vec(vec![0x00, 0x00, 0x00, 0x00]),
  /// #   SpiTransaction::transaction_end(),
  /// # ]);
  /// # use p9813::P9813;
  /// let mut p9813 = P9813::new(spi);
  /// p9813.set_color(0, 255, 200)?;
  /// # let mut spi = p9813.release();
  /// # spi.done();
  /// # Ok(())
  /// # }
  /// ```
  pub fn set_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), SPI::Error> {
    self.set_colors([(r, g, b)])
  }

  /// Set colors for multiple P9813s chained together.
  ///
  /// ```
  /// # fn main() -> Result<(), embedded_hal::spi::ErrorKind> {
  /// # use embedded_hal_mock::eh1::{spi::{Mock as SpiMock, Transaction as SpiTransaction}, delay::NoopDelay};
  /// # let spi = SpiMock::new(&[
  /// #   // Start frame.
  /// #   SpiTransaction::transaction_start(),
  /// #   SpiTransaction::write_vec(vec![0x00, 0x00, 0x00, 0x00]),
  /// #   SpiTransaction::transaction_end(),
  /// #
  /// #   // Set color.
  /// #   SpiTransaction::transaction_start(),
  /// #   SpiTransaction::write_vec(vec![0b11000011, 200, 255, 0]),
  /// #   SpiTransaction::transaction_end(),
  /// #
  /// #   // Set color.
  /// #   SpiTransaction::transaction_start(),
  /// #   SpiTransaction::write_vec(vec![0b11111100, 20, 50, 255]),
  /// #   SpiTransaction::transaction_end(),
  /// #
  /// #   // End frame.
  /// #   SpiTransaction::transaction_start(),
  /// #   SpiTransaction::write_vec(vec![0x00, 0x00, 0x00, 0x00]),
  /// #   SpiTransaction::transaction_end(),
  /// # ]);
  /// # use p9813::P9813;
  /// let mut p9813 = P9813::new(spi);
  /// p9813.set_colors([(0, 255, 200), (255, 50, 20)])?;
  /// # let mut spi = p9813.release();
  /// # spi.done();
  /// # Ok(())
  /// # }
  /// ```
  pub fn set_colors(&mut self, colors: impl AsRef<[(u8, u8, u8)]>) -> Result<(), SPI::Error> {
    self.spi.write(&FRAME_START)?;

    for &(r, g, b) in colors.as_ref().iter() {
      self.spi.write(&color_to_array(r, g, b))?;
    }

    self.spi.write(&FRAME_END)
  }
}
