#![no_main]
#![no_std]



use space_war as _;

use space_war::{
    types::Display,
    game::*,
    objects::*,
    Node,
    List,
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
        disp : RefCell<Option<Display>>,
        player:Player,
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
        disp.set_rotation(DisplayRotation::Rotate270).unwrap();


        // Used RefCell due to RTIC currently doesn't impliments double object lock at a time
        let disp = RefCell::new(Some(disp));

        let player = space_war::game_init();
        init::LateResources{ disp, player, delay}
    }



    #[idle(resources = [&disp, player, delay])]
    fn idle( mut c: idle::Context)->!{
        // it is the border of display
        let border = Rectangle::new(
            Point::zero(), Point::new(
                (space_war::DISPLAY_WIDTH - 1 ) as i32,
                (space_war::DISPLAY_HEIGHT - 1) as i32
            ))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
        let mut display = c.resources.disp.replace(None).unwrap();
        // let mut delay = c.resources.delay;
        let mut enemies:space_war::List<'_, Enemy> = List::new() ;
        let mut bullets = List::new();
        let asteroids:Option<Node<Asteroids>> = None;
        let enemies_count = 5;


        // bullets creation
        // TODO: later it should be replaced with macros
        let bullet = &Node::new(
            c.resources.player.lock(|player:&mut Player|{
                player.shoot()
            }),
            bullets.head.take(),
        );
        bullets.push(
            bullet
        );
        loop{
            let enemy;
            if enemies_count > 0{
                enemy = Node::new(
                    Enemy::new(63, 1, &ENEMY),
                    enemies.head.take()
                    );
                enemies.push(&enemy);
            };
            
            display.clear();
            c.resources.player.lock(|player:&mut Player|{
                player.update();
                player.draw(&mut display);
            });

            
            bullets.update();
            enemies.update();
            bullets.draw(&mut display);
            enemies.draw(&mut display);
            border.draw(&mut display).unwrap();
            display.flush().unwrap();
            // delay.lock(|delay|{
            //     delay.delay_ms(1000/space_war::FPS_LIMIT);
            // });
        }
    }
}
