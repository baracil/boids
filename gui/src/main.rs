use raylib::prelude::*;

use gui::gui::create_gui;
use gui::gui::Gui;


use gui::widget_operation::{WidgetOp};

use gui::alignment::Alignment;
use gui::alignment::VAlignment::Bottom;
use gui::alignment::HAlignment::Left;

use tree::tree::Tree;




fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .vsync()
        .resizable()
        .msaa_4x()
        .title("Hello, World")
        .build();

    let mut gui = create_gui();

    let font_id = gui
        .load_font(
            &mut rl,
            &thread,
            "/home/Bastien Aracil/Downloads/FreckleFace-Regular.ttf",
            48,
            200,
        )
        .unwrap();

    let n = gui.create_label(|par| -> () {
        par.set_text("Hello".to_owned())
            .set_font_id(font_id)
            .set_padding(10.0)
            .set_position(
                &Vector2 { x: 120.0, y: 120.0 },
                Alignment {
                    vertical: Bottom,
                    horizontal: Left,
                },
            );
    });

    gui.set_root(n);
    gui.layout();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let screen_width = d.get_screen_width();
        let screen_height = d.get_screen_height();

        d.clear_background(Color::WHITE);

        gui.render(&mut d);

        d.draw_line(0, 120, screen_width, 120, Color::RED);
        d.draw_line(120, 0, 120, screen_height, Color::RED);
    }
}
