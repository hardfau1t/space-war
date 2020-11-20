use crate::game::Sprite;
// these are raw bytes of sprites
pub static PLAYER_1_SPRITE : Sprite = Sprite{
    data    :   &[0x04, 0x00, 0x04, 0x00, 0x0e, 0x00, 0x1f, 0x00, 0x3f, 0x80, 0x7f, 0xc0, 0xee, 0xe0, 0x9f, 0x20, 0x35, 0x80, 0x20, 0x80],
    width   :   11,
    height  :   10,
};
pub static PLAYER_2_SPRITE : Sprite = Sprite{
    data    :   &[0x04, 0x00, 0x04, 0x00, 0x0e, 0x00, 0x1f, 0x00, 0x3f, 0x80, 0x7f, 0xc0, 0xff, 0xe0, 0xff, 0xe0, 0x7f, 0xc0, 0x2e, 0x80],
    width   :   11,
    height  :   10,
};

// pub static BULLET_SPRITE : Sprite = Sprite{
//     data    : &[0x70, 0xf8, 0xf8, 0xf8, 0x70],
//     width   : 5,
//     height  :5
// };

pub static BULLET_SPRITE : Sprite = Sprite{
    data    : &[0x40, 0xe0, 0xe0],
    width   : 3,
    height  :3
};

pub static ENEMY_SPRITE :Sprite = Sprite{
    data    : &[0xff, 0xc0, 0x80, 0x40, 0xa1, 0x40, 0x9e, 0x40, 0xa3, 0x40, 0xa1, 0x40, 0xa1, 0x40, 0x80, 0x40, 0x80, 0x40, 0xff, 0xc0],
    width   : 10,
    height  : 10,
};

pub static ASTEROID_SPRITE: Sprite = Sprite{
    data    : &[0x3f, 0x40, 0x7b, 0x00, 0xff, 0xc0, 0xff, 0xc0, 0xbf, 0xc0, 0xfb, 0xc0, 0xff, 0x80, 0xfe, 0x00, 0x7e, 0xc0, 0x3c, 0xc0],
    width   : 10,
    height  : 10,
};

// Constants
pub const FPS_LIMIT:u16         = 10;
pub const LEVEL_SCORE:u8        = 40;
pub const PLAYER_COOL_DOWN:u8   = 10;
