/// Exercise 5:
/// Add the library "apricity" to the project and create a window.
///
/// > cargo add apricity --git https://github.com/MindroadGabriel/apricity.git
/// > cargo add ttf-noto-sans
///
/// Use the helper functions in draw_geo to load the countries data and use the geometry inside
/// to draw a world map.
/// Use the code from exercise 4 to draw a circle on each city that is largest in its country.
/// Draw some text to the screen.
/// Print to console when click events happen.
///
/// Useful snippets:
///     let world_map: SimpleImage = draw_geo::create_world_map(window_width, window_height).unwrap();
///     let font: Font<'_> = Font::try_from_bytes(ttf_noto_sans::REGULAR).unwrap();
///     let screen_text: SimpleImage = SimpleWindow::create_text_image(font: &Font<'static>, "Some Text", font_size, color).unwrap();
///     window.run(|window, events| { ... });
///     draw_geo::draw_image(window, &image, position_on_screen, Alignment::Left);

use apricity::{Coordinate, Point, gui::*};
use rust_course_2025::helpers::exercise_5::{ city_parser::*, draw_geo::*};

const WINDOW_WIDTH: u32 = 1500;
const WINDOW_HEIGHT: u32 = 750;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

