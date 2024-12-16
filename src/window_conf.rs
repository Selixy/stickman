use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Stickman Draw".to_string(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}
