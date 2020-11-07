#![no_main]
#![no_std]



use space_war as _;

use space_war::types::Display;

use rtic::app;

use stm32f7xx_hal::{
    prelude::*,
    i2c::{BlockingI2c, self},
};

use embedded_graphics::{
    prelude::*,
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    style::TextStyleBuilder,
};

use ssd1306::{
    prelude::*,
    I2CDIBuilder,
    Builder,
};

#[app(device = stm32f7xx_hal::pac, peripherals=true)]
mod app {
    use super::*;
    #[resources]
    struct Resources {
        disp : Display,
    }
    #[init]
    fn init(c : init::Context)->init::LateResources {
        let mut rcc = c.device.RCC.constrain();
        let gpiof : stm32f7xx_hal::gpio::gpiof::Parts = c.device.GPIOF.split();
        let sda = gpiof.pf0.into_alternate_af4().set_open_drain();
        let scl = gpiof.pf1.into_alternate_af4().set_open_drain();
        let clk = rcc.cfgr.sysclk(32.mhz()).freeze();
        let i2c_display = BlockingI2c::i2c2(c.device.I2C2, (scl, sda), i2c::Mode::FastPlus{ frequency: 400_000.hz() }, clk, &mut rcc.apb1, 999);
        let interface = I2CDIBuilder::new().init(i2c_display);
        let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();
        disp.init().expect("couldn't initiate display");
        init::LateResources{ disp}
    }


    #[idle(resources = [disp])]
    fn idle(mut c : idle::Context)->!{
        let text_style = TextStyleBuilder::new(Font6x8).text_color(BinaryColor::On).build();
        c.resources.disp.lock(| disp:&mut Display |{
            disp.clear(); 
            Text::new("something smells new", Point::zero())
                .into_styled(text_style)
                .draw(disp).unwrap();
            disp.flush().unwrap();
        });
        defmt::info!("idle");

        space_war::exit();
    }
}

