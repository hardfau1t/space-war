use ssd1306::prelude::*;
use stm32f7xx_hal::{
    pac::I2C2,
    gpio::{
        Alternate,
        gpiof::{
            PF0,
            PF1,
        }
    },

};

pub type Display = ssd1306::mode::GraphicsMode<I2CInterface<stm32f7xx_hal::i2c::BlockingI2c<I2C2, PF1<Alternate<stm32f7xx_hal::gpio::AF4>>, PF0<Alternate<stm32f7xx_hal::gpio::AF4>>>>>;
