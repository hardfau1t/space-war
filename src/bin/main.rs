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
    GamePool,
};

use rtic::app;

use stm32f7xx_hal as _; 
use stm32f7xx_hal::{
    prelude::*,
    i2c::{BlockingI2c, self},
    delay::Delay,
    gpio::{Edge, ExtiPin},
    timer::{Timer, Event},
    pac::{EXTI, TIM2},
    rng::Rng,
};

use ssd1306::{
    prelude::*,
    I2CDIBuilder,
    Builder,
};

#[app(device = stm32f7xx_hal::pac, peripherals=true, dispatchers = [SPI4, SPI5])]
mod app {
    use super::*;
    #[resources]
    struct Resources {
        disp : Display,
        game : GamePool,
        delay: Delay,
        direct:(Left, Right), // direction
        shoot: ButtonShoot,
        exti : EXTI,
        timer2: Timer<TIM2>,
        rng: Rng,
        pause: Pause,
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
        let left = gpiof.pf9.into_pull_up_input();
        let right = gpiof.pf8.into_pull_up_input();
        let mut shoot = gpiof.pf2.into_pull_up_input();
        shoot.make_interrupt_source(&mut syscfg, &mut rcc) ;
        shoot.trigger_on_edge(&mut exti, Edge::FALLING);
        shoot.enable_interrupt(&mut exti);

        // pause button
        let mut pause = gpiof.pf6.into_pull_up_input();
        pause.make_interrupt_source(&mut syscfg, &mut rcc);
        pause.trigger_on_edge(&mut exti, Edge::FALLING);
        // NOTE: pause currently disabled
        // pause.enable_interrupt(&mut exti);

        let rng = c.device.RNG.init();
        let mut rcc = rcc.constrain();
        // if clock is changed need to change timer delay too,
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

        // initialize timer for player ammunition
        let mut fps_timer = Timer::tim2(c.device.TIM2, 1.hz(), clk, &mut rcc.apb1 );
        fps_timer.listen(Event::TimeOut);

        // set log level
        let game = GamePool::init(&disp);
        init::LateResources{ disp, game, delay, direct:(left, right), 
            shoot, exti, timer2:fps_timer, rng, pause,
        }
    }

    #[idle(resources = [rng,disp, game, delay, &direct])]
    fn idle( c: idle::Context)->!{
        // it is the border of display
        let direct = c.resources.direct;
        let mut game = c.resources.game;
        let mut display = c.resources.disp;
        let mut rng = c.resources.rng;
        loop{
            game.lock(|game|{
                rng.lock(|rng|{
                    game.spawn(rng);
                });
                game.update(&direct);
                game.collect();
                display.lock(|display:&mut Display|{
                    display.clear();
                    game.draw(display);
                    game.draw_stats(display);
                    display.flush().unwrap();
                });
                if game.is_ok(){
                    game_over::spawn().unwrap();
                };
            });
        }
    }

    #[task(binds = EXTI2, resources = [shoot, game, exti, timer2], priority = 2)]
    fn exti2(c: exti2::Context){
        let mut game = c.resources.game;
        let mut shoot = c.resources.shoot;

        // spawn a bullet
        shoot.lock(|button:&mut ButtonShoot|{
            // clear the interrput first
            button.clear_interrupt_pending_bit();
            game.lock(|game:&mut GamePool|{
                if game.player.can_shoot(){
                    game.player.shoot();
                }
            });
        });
    }

    #[task(resources = [game, disp, delay], priority = 5)]
    fn game_over(c:game_over::Context){
        let mut game = c.resources.game;
        let score = game.lock(|game|{
            game.player.player_score
        });
        let mut display = c.resources.disp;
        let mut delay = c.resources.delay;
        display.lock(|display|{
            delay.lock(|delay|{
                space_war::final_screen(score, display, delay);
            })
        })
    }

    // pause has lower priority than game over state,
    // so that you can't pause in game over state only reset
    // and has higher priority than others so that no other interrupt can pull
    // out of pause state except pause.
    #[task(binds = EXTI9_5, resources = [disp, delay, pause], priority = 4)]
    fn pause(c: pause::Context){
        let mut disp = c.resources.disp;
        let mut intr = c.resources.pause;

        intr.lock(| pin:&mut Pause|{
            pin.clear_interrupt_pending_bit();
        });
        static mut PAUSED:bool = true;
        // SAFETY: safe usage of static local variable
        let paused = unsafe{
             &mut PAUSED
        };


        if *paused{
            disp.lock(|display|{
                space_war::display_pause(display);
            });
            *paused = false;
        rtic::pend(stm32f7xx_hal::interrupt::EXTI9_5);
            loop{
            }
        } else{
            defmt::debug!("pause remove");
            *paused = true;
        }

        // pause will spawn empty loop so that only pause intr and reset can pull out from it
    }
    
    // after paused this loop will continue, only pause and reset can break this 
    #[task(priority = 3)]
    fn empty_loop(_: empty_loop::Context){
        defmt::debug!("state paused");
        rtic::pend(stm32f7xx_hal::interrupt::EXTI9_5);
    }

    #[task(binds = TIM2, resources = [shoot, game, exti, timer2], priority = 2)]
    fn tim2(c: tim2::Context){
        let mut game = c.resources.game;
        let mut timer = c.resources.timer2;
        game.lock(|game:&mut GamePool|{
            game.set_fps();
        });
        // clear interrupt
        timer.lock(|timer:&mut Timer<TIM2>|{
            timer.clear_interrupt(Event::TimeOut);
            timer.unlisten(Event::TimeOut);
        });
    }
}
