use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use smart_leds::{RGB8, SmartLedsWrite};
use std::{env, error::Error};
use ws2812_spi::Ws2812;

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(color) = env::args()
        .nth(1)
        .and_then(|hex| u32::from_str_radix(&hex, 16).ok())
    {
        let color = RGB8::new((color >> 16) as u8, (color >> 8) as u8, color as u8);
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 4_200_000, Mode::Mode0)?;
        Ws2812::new(spi).write([color])?;
    }
    Ok(())
}
