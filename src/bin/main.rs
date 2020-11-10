#![no_main]
#![no_std]



use space_war as _;

use space_war::{
    types::Display,
    game::Object,
};

use core::cell::RefCell;

use rtic::app;

use stm32f7xx_hal::{
    prelude::*,
    i2c::{BlockingI2c, self},
    delay::Delay,
};

use embedded_graphics::{
    prelude::*,
    primitives::Rectangle,
    style::PrimitiveStyle,
    pixelcolor::BinaryColor,
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
        disp : RefCell<Display>,
        entities:[Object;2],
        delay: Delay,
    }
    #[init]
    fn init(c : init::Context)->init::LateResources {
        let mut rcc = c.device.RCC.constrain();
        let gpiof : stm32f7xx_hal::gpio::gpiof::Parts = c.device.GPIOF.split();
        let sda = gpiof.pf0.into_alternate_af4().set_open_drain();
        let scl = gpiof.pf1.into_alternate_af4().set_open_drain();
        let clk = rcc.cfgr.sysclk(32.mhz()).freeze();
        let syst = c.core.SYST;

        // delay object so that game is not too fast
        let delay = Delay::new(syst, clk);

        // initilize Display with i2c
        let i2c_display = BlockingI2c::i2c2(c.device.I2C2, (scl, sda), i2c::Mode::FastPlus{ frequency: 400_000.hz() }, clk, &mut rcc.apb1, 999);
        let interface = I2CDIBuilder::new().init(i2c_display);
        let mut disp:GraphicsMode<_>= Builder::new().connect(interface).into();
        disp.init().expect("couldn't initiate display");

        // adding background border
        Rectangle::new(
            Point::zero(), Point::new(
                (space_war::DISPLAY_WIDTH - 1 ) as i32,
                (space_war::DISPLAY_HEIGHT - 1) as i32
            ))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut disp).unwrap();

        // Used RefCell due to RTIC currently doesn't impliments double object lock at a time
        let disp = RefCell::new(disp);

        let entities = space_war::game_init();
        init::LateResources{ disp, entities, delay}
    }


    #[idle(resources = [&disp, entities, delay])]
    fn idle( mut c: idle::Context)->!{
        let disp = c.resources.disp;
        let mut delay = c.resources.delay;
        let mut cntr = 0;
        loop{
            c.resources.entities.lock(|[mut player, mut opponent]:&mut[Object;2]|{
                space_war::game_update(&mut player, &mut opponent);
                    space_war::game_draw(&player, &opponent, &mut disp.borrow_mut());
            });
            delay.lock(|delay|{
                delay.delay_ms(1000/space_war::FPS_LIMIT);
            });
            defmt::debug!(" frame number {:?}",  cntr);
            cntr +=1;
        }
    }
}

