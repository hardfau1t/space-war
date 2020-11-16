#![no_main]
#![no_std]

// PF0  : SDA
// PF1  : SCL
// PF2  : Left
// PF8  : Shoot
// PF9  : Rigt

use space_war as _;

use space_war::{
    types::*,
    GameObject,
    game::Shooter,
};

use core::cell::RefCell;

use rtic::app;

use stm32f7xx_hal::{
    prelude::*,
    i2c::{BlockingI2c, self},
    delay::Delay,
    gpio::{Edge, ExtiPin},
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
        disp : RefCell<Option<Display>>,
        game : space_war::GameObject,
        delay: Delay,
        direct:(Left, Right), // direction
        shoot: ButtonShoot,
    }
    #[init]
    fn init(c : init::Context)->init::LateResources {
        let mut rcc = c.device.RCC;
        let gpiof : stm32f7xx_hal::gpio::gpiof::Parts = c.device.GPIOF.split();
        let mut syscfg = c.device.SYSCFG;
        let mut exti = c.device.EXTI;

        // pins assigning
        let sda = gpiof.pf0.into_alternate_af4().set_open_drain();
        let scl = gpiof.pf1.into_alternate_af4().set_open_drain();
        let left = gpiof.pf2.into_pull_up_input();
        let right = gpiof.pf9.into_pull_up_input();
        let mut shoot = gpiof.pf8.into_pull_up_input();
        shoot.make_interrupt_source(&mut syscfg, &mut rcc) ;
        shoot.trigger_on_edge(&mut exti, Edge::FALLING);
        shoot.enable_interrupt(&mut exti);

        let mut rcc = rcc.constrain();
        let clk = rcc.cfgr.sysclk(32.mhz()).freeze();
        let syst = c.core.SYST;

        // delay object so that game is not too fast
        let delay = Delay::new(syst, clk);

        // initilize Display with i2c
        let i2c_display = BlockingI2c::i2c2(c.device.I2C2, (scl, sda), i2c::Mode::FastPlus{ frequency: 400_000.hz() }, clk, &mut rcc.apb1, 999);
        let interface = I2CDIBuilder::new().init(i2c_display);
        let mut disp:GraphicsMode<_>= Builder::new().connect(interface).into();
        disp.init().expect("couldn't initiate display");
        disp.set_rotation(DisplayRotation::Rotate270).unwrap();


        // Used RefCell due to RTIC currently doesn't impliments double object lock at a time
        let disp = RefCell::new(Some(disp));

        let game = GameObject::init();
        init::LateResources{ disp, game, delay, direct:(left, right), shoot}
    }

    #[idle(resources = [&disp, game, delay, &direct])]
    fn idle( mut c: idle::Context)->!{
        // it is the border of display
        let direct = c.resources.direct;
        let mut display = c.resources.disp.replace(None).unwrap();
        loop{
            display.clear();
            c.resources.game.lock(|game|{
                game.update(&direct);
                game.draw(&mut display);
            });
            display.flush().unwrap();
        }
            // delay.lock(|delay|{
            //     delay.delay_ms(1000/space_war::FPS_LIMIT);
            // });
    }

    #[task(binds = EXTI9_5, resources = [shoot, game], priority = 2)]
    fn exti9_5(mut c: exti9_5::Context){
        // spawn a bullet
        c.resources.game.lock(|game|{
            game.bullets.push(game.player.shoot()).expect("cant create more bullets");
        });

        // clear interrupt
        c.resources.shoot.lock(|button|{
            button.clear_interrupt_pending_bit();
        });
    }
}

