#![no_main]
#![no_std]
#![allow(dead_code)]


use space_war as _;

use space_war::types::Display;

use rtic::app;

use stm32f7xx_hal::{
    prelude::*,
    i2c::{BlockingI2c, self},
};

use embedded_graphics::{
    prelude::*,
    image::{
        ImageRaw,
        Image,
    },
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    style::{ PrimitiveStyle, TextStyleBuilder },
    primitives::{
        Rectangle,
    },
};

use ssd1306::{
    prelude::*,
    I2CDIBuilder,
    Builder,
    displayrotation::DisplayRotation,
};

#[app(device = stm32f7xx_hal::pac, peripherals=true)]
mod app {
    use super::*;
    #[resources]
    struct Resources {
        disp : Display,
        width:u8,
        height:u8,
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
        disp.set_rotation(DisplayRotation::Rotate270).unwrap();
        let (width, height) = disp.get_dimensions();
        init::LateResources{ disp, width, height}
    }


    #[idle(resources = [disp, &width, &height])]
    fn idle(mut c : idle::Context)->!{
        let width = c.resources.width;
        let height = c.resources.height;
        let byte_array_1 = [0x04, 0x00, 0x04, 0x00, 0x0e, 0x00, 0x1f, 0x00, 0x3f, 0x80, 0x7f, 0xc0, 0xee, 0xe0, 0x9f, 0x20, 0x35, 0x80, 0x20, 0x80];
        let byte_array_2 = [0x04, 0x00, 0x04, 0x00, 0x0e, 0x00, 0x1f, 0x00, 0x3f, 0x80, 0x7f, 0xc0, 0xff, 0xe0, 0xff, 0xe0, 0x7f, 0xc0, 0x2e, 0x80];
        let image_width = 11i32;
        let image_height = 10i32;
        let raw_1: ImageRaw<BinaryColor> = ImageRaw::new(&byte_array_1, image_width as u32, image_height as u32);
        let raw_2: ImageRaw<BinaryColor> = ImageRaw::new(&byte_array_2, image_width as u32, image_height as u32);
        let y = *height as i32 - image_height - 1;
        let x = (*width as i32 / 2) - (image_width / 2) - 1;
        loop{
            c.resources.disp.lock(| disp:&mut Display |{
                disp.clear(); 
                draw_rect(Point::zero(),Point::new((*width-1) as i32,(*height-1) as i32), disp);
                // draw_rect(Point::new(118, 35), Point::new(126, 36), disp);
                draw_image(x, y, &raw_1, disp );
                disp.flush().unwrap();
            });
            c.resources.disp.lock(| disp:&mut Display |{
                disp.clear(); 
                draw_rect(Point::zero(),Point::new((*width-1) as i32,(*height-1) as i32), disp);
                // draw_rect(Point::new(118, 35), Point::new(126, 36), disp);
                draw_image(x, y, &raw_2, disp );
                disp.flush().unwrap();
            });
        }
    }
}


fn draw_rect(p1:Point, p2:Point, disp: &mut Display){
    Rectangle::new(p1, p2)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(disp)
        .unwrap();
}

fn draw_image(x:i32, y:i32, image:&ImageRaw<BinaryColor>, disp:&mut Display){


    let im = Image::new(
        image,
        Point::new(x, y),
        // Point::new((w/2) as i32 - (image_width/2) as i32 - 1, h as i32 - image_height as i32 -1),
    );
    im.draw(disp).unwrap();
}

fn draw_text(text:&str, disp:&mut Display){
    let text_style = TextStyleBuilder::new(Font6x8).text_color(BinaryColor::On).build();
    Text::new(text, Point::zero())
        .into_styled(text_style)
        .draw(disp).unwrap();
}
