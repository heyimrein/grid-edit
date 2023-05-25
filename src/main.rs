use macroquad::prelude::*;


#[macroquad::main(window_conf)]
async fn main() {
    let win = window_conf();
    let mut cam = Camera::new()
        .origin_screen_pos(vec2((win.window_width / 2) as f32, (win.window_height / 2) as f32));

    loop {
        clear_background(WHITE);


        // Draw origin markers
        draw_line(
            0.,
            cam.origin_screen_pos.y,
            win.window_width as f32,
            cam.origin_screen_pos.y,
            1.,
            GRAY
        );
        draw_line(
            cam.origin_screen_pos.x,
            0.,
            cam.origin_screen_pos.x,
            win.window_height as f32,
            1.,
            GRAY
        );
        draw_circle(
            cam.origin_screen_pos.x,
            cam.origin_screen_pos.y,
            2.,
            BLACK
        );

        next_frame().await;
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: String::from("title"),
        window_width: 600,
        window_height: 600,
        ..Default::default()
    }
}


struct Camera {
    origin_screen_pos: Vec2,
    unit_pixel_size: u16
}

impl Camera {
    fn new() -> Self {
        Camera {
            origin_screen_pos: vec2(0., 0.),
            unit_pixel_size: 32
        }
    }

    /// Set `origin_screen_pos` to `value`.
    fn origin_screen_pos(mut self: Self, value: Vec2) -> Self {
        self.origin_screen_pos = value;
        return self;
    }
}
