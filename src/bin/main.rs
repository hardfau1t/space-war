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
        let disp: RefCell<GraphicsMode<_>>= RefCell::new(Builder::new().connect(interface).into());
        disp.borrow_mut().init().expect("couldn't initiate display");

        let entities = space_war::game_init();
        init::LateResources{ disp, entities}
    }


    #[idle(resources = [&disp, entities])]
    fn idle( mut c: idle::Context)->!{
        loop{
            let disp = c.resources.disp;
            c.resources.entities.lock(|[mut player, mut opponent]:&mut[Object;2]|{
                space_war::game_update(&mut player, &mut opponent);
                    space_war::game_draw(&mut player, &mut opponent, &mut disp.borrow_mut());
            });
            defmt::debug!("Exiting Succesfully");
            space_war::exit();
        }
    }
}

