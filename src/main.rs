use macroquad::{color::{BLACK, RED, WHITE}, input::{MouseButton, is_mouse_button_down, mouse_position}, math, shapes::draw_rectangle_lines, text::draw_text, ui::{self, root_ui}, window::{Conf, clear_background, next_frame}};

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

const SQUARE_DEFAULT_WIDTH: f32 = 100.;
const SQUARE_DEFAULT_HEIGHT: f32 = 100.;
const SQUARE_BORDER_THICKNESS: f32 = 10.; 

fn window_conf() -> Conf {
    Conf {
        window_title: "floating".to_owned(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        fullscreen: false,
        window_resizable: false,
        high_dpi: false,
        sample_count: 1,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut square_a_position: math::Vec2 = math::Vec2::ZERO;
    let square_a_size: math::Vec2 = math::Vec2::new(
        SQUARE_DEFAULT_WIDTH,
        SQUARE_DEFAULT_HEIGHT
    );

    let mut square_b_position: math::Vec2 = math::Vec2::new(100., 100.);
    let square_b_size: math::Vec2 = math::Vec2::new(
        SQUARE_DEFAULT_WIDTH,
        SQUARE_DEFAULT_HEIGHT
    );

    let mut is_following_square_a = true;

    let mut previous_mouse_position = math::Vec2::from(mouse_position());

    loop {
        clear_background(WHITE);

        let current_mouse_position = math::Vec2::from(mouse_position());

        let relative_mouse_from_last_frame = current_mouse_position - previous_mouse_position;

        if is_mouse_button_down(MouseButton::Left) {
            if is_following_square_a {
                square_a_position = square_a_position + relative_mouse_from_last_frame;
            } else {
                square_b_position = square_b_position + relative_mouse_from_last_frame;
            }
        }

        previous_mouse_position = math::Vec2::from(mouse_position());

        draw_rectangle_lines(
            square_a_position.x,
            square_a_position.y,
            square_a_size.x,
            square_a_size.y,
            SQUARE_BORDER_THICKNESS,
            BLACK
        );

        draw_text(
            "A",
            square_a_position.x + 20.,
            square_a_position.y + 40.,
            40.,
            BLACK
        );

        draw_rectangle_lines(
            square_b_position.x,
            square_b_position.y,
            square_b_size.x,
            square_b_size.y,
            SQUARE_BORDER_THICKNESS,
            RED
        );

        draw_text(
            "B",
            square_b_position.x + 20.,
            square_b_position.y + 40.,
            40.,
            RED
        );

        let are_squares_colliding = are_squares_colliding(
            square_a_position,
            square_a_size,
            square_b_position,
            square_b_size
        );

        draw_text(are_squares_colliding.to_string(), (SCREEN_WIDTH - 80) as f32, 20., 24., BLACK);

        ui::widgets::Checkbox::new(0)
            .label("follow black square?")
            .pos(math::Vec2::ZERO)
            .ui(&mut *root_ui(), &mut is_following_square_a);

        next_frame().await
    }
}

fn are_squares_colliding(
    a_position: math::Vec2,
    a_size: math::Vec2,
    b_position: math::Vec2,
    b_size: math::Vec2
) -> bool {
    // TODO: abstract squares into structs with get_left_side_position, get_right_side_position methods
    let is_a_to_the_right_of_b = a_position.x > (b_position.x + b_size.x);
    let is_a_to_the_left_of_b = (a_position.x + a_size.x) < b_position.x;
    let is_a_above_b = (a_position.y + a_size.y) < b_position.y;
    let is_a_below_b = a_position.y > (b_position.y + b_size.y);

    !(is_a_to_the_right_of_b || is_a_to_the_left_of_b || is_a_above_b || is_a_below_b)
}
