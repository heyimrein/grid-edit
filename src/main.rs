use macroquad::prelude::*;


#[macroquad::main(window_conf)]
async fn main() {
    let win = window_conf();
    let mut cam = Camera::new()
        .origin_screen_pos(vec2((win.window_width / 2) as f32, (win.window_height / 2) as f32));

    let mut mouse_pos = mouse_position();
    loop {
        clear_background(WHITE);

        // Input handling
        let cur_mouse_pos = mouse_position();
        let mouse_pos_delta = (cur_mouse_pos.0 - mouse_pos.0, cur_mouse_pos.1 - mouse_pos.1);

        if is_mouse_button_down(MouseButton::Right) {
            cam.origin_screen_pos += vec2(
                mouse_pos_delta.0,
                mouse_pos_delta.1
            );
        }

        mouse_pos = cur_mouse_pos;

        // Grid rendering, assumes that the origin in on-screen and isn't very concise or optimized
        // The grid is properly rendered when origin in off-screen, but grid cells off-screen in the
        // origin's direction are (probably) still handled. Not performant! 😭
        // TODO: Improve robustness
        let pos_dif_down = win.window_height as f32 - cam.origin_screen_pos.y;
        for y in 0..(pos_dif_down / cam.unit_pixel_size as f32).floor() as i32 {
            let y_pos = cam.origin_screen_pos.y + cam.unit_pixel_size as f32 * (y + 1) as f32 + 0.5;
            draw_line(
                0.,
                y_pos,
                win.window_width as f32,
                y_pos,
                1.,
                LIGHTGRAY
            );
        }
        drop(pos_dif_down);

        for y in 0..(cam.origin_screen_pos.y / cam.unit_pixel_size as f32).floor() as i32 {
            let y_pos = cam.origin_screen_pos.y - cam.unit_pixel_size as f32 * (y + 1) as f32 + 0.5;
            draw_line(
                0.,
                y_pos,
                win.window_width as f32,
                y_pos,
                1.,
                LIGHTGRAY
            );
        }

        let pos_dif_right = win.window_width as f32 - cam.origin_screen_pos.x;
        for x in 0..(pos_dif_right / cam.unit_pixel_size as f32).floor() as i32 {
            let x_pos = cam.origin_screen_pos.x + cam.unit_pixel_size as f32 * (x + 1) as f32 + 0.5;
            draw_line(
                x_pos,
                0.,
                x_pos,
                win.window_height as f32,
                1.,
                LIGHTGRAY
            );
        }
        drop(pos_dif_right);

        for y in 0..(cam.origin_screen_pos.x / cam.unit_pixel_size as f32).floor() as i32 {
            let x_pos = cam.origin_screen_pos.x - cam.unit_pixel_size as f32 * (y + 1) as f32 + 0.5;
            draw_line(
                x_pos,
                0.,
                x_pos,
                win.window_height as f32,
                1.,
                LIGHTGRAY
            );
        }

        // Draw origin markers
        draw_line(
            0.,
            cam.origin_screen_pos.y.round(),
            win.window_width as f32,
            cam.origin_screen_pos.y.round(),
            2.,
            RED
        );
        draw_line(
            cam.origin_screen_pos.x.round(),
            0.,
            cam.origin_screen_pos.x.round(),
            win.window_height as f32,
            2.,
            DARKGREEN
        );
        draw_circle(
            cam.origin_screen_pos.x.round(),
            cam.origin_screen_pos.y.round(),
            2.,
            BLACK
        );

        next_frame().await;
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Grid Edit"),
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
