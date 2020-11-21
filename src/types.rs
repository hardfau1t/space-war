use ssd1306::prelude::*;
use stm32f7xx_hal::{
    pac::I2C2,
    gpio::{
        Alternate, Input, PullUp,
        gpiof::{ PF0, PF1, PF2, PF9, PF8 , PF6}
    },

};

pub type Display = ssd1306::mode::GraphicsMode<I2CInterface<stm32f7xx_hal::i2c::BlockingI2c<I2C2, PF1<Alternate<stm32f7xx_hal::gpio::AF4>>, PF0<Alternate<stm32f7xx_hal::gpio::AF4>>>>>;
pub type Left = PF9<Input<PullUp>>;
pub type Right = PF8<Input<PullUp>>;
pub type ButtonShoot = PF2<Input<PullUp>>;
pub type Delay = stm32f7xx_hal::delay::Delay;
pub type Pause = PF6<Input<PullUp>>;
