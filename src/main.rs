use macroquad::{color::{BLACK, RED, WHITE}, math, shapes::draw_rectangle_lines, window::{Conf, clear_background, next_frame}};

const SQUARE_DEFAULT_WIDTH: f32 = 100.;
const SQUARE_DEFAULT_HEIGHT: f32 = 100.;
const SQUARE_BORDER_THICKNESS: f32 = 10.; 

fn window_conf() -> Conf {
    Conf {
        window_title: "floating".to_owned(),
        window_width: 640,
        window_height: 480,
        fullscreen: false,
        window_resizable: false,
        high_dpi: false,
        sample_count: 1,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let square_1_position: math::Vec2 = math::Vec2::ZERO;
    let square_1_size: math::Vec2 = math::Vec2::new(
        SQUARE_DEFAULT_WIDTH,
        SQUARE_DEFAULT_HEIGHT
    );

    let square_2_position: math::Vec2 = math::Vec2::new(100., 100.);
    let square_2_size: math::Vec2 = math::Vec2::new(
        SQUARE_DEFAULT_WIDTH,
        SQUARE_DEFAULT_HEIGHT
    );

    loop {
        clear_background(WHITE);

        draw_rectangle_lines(
            square_1_position.x,
            square_1_position.y,
            square_1_size.x,
            square_1_size.y,
            SQUARE_BORDER_THICKNESS,
            BLACK
        );

        draw_rectangle_lines(
            square_2_position.x,
            square_2_position.y,
            square_2_size.x,
            square_2_size.y,
            SQUARE_BORDER_THICKNESS,
            RED
        );

        next_frame().await
    }
}
