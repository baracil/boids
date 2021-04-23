use raylib::prelude::*;

use gui::gui::Gui;


use gui::widget_operation::{WidgetOp};

use gui::alignment::VAlignment::{Bottom, Top, Center};
use gui::alignment::HAlignment::{Left, Right, Middle};

use gui::widget::Widget::Pane;
use gui::pane::PanePar;
use gui::size::Size;


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .vsync()
        .resizable()
        .msaa_4x()
        .title("Hello, World")
        .build();

    let mut gui = Gui::new();

    let pane = {
        let par = PanePar::new();
        par.set_requested_height(&gui, 200.0)
            .set_requested_width(&gui, 200.0)
            .set_absolute_coordinate_x(&gui, false)
            .set_absolute_coordinate_y(&gui, false)
            .set_position(&gui, 50.0, 50.0)
            .set_valignment(&gui, Center)
            .set_halignment(&gui, Middle);
        Pane(par)
    };

    gui.insert_root(pane);

    gui.load_font(&mut rl, &thread,
                                "default",
                                "/home/Bastien Aracil/Downloads/FreckleFace-Regular.ttf",
                                48,
                                200,
    ).expect("Could not load the font");


    let mut screen_size: Size = Size::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if d.is_window_resized() {
            screen_size = Size::new(d.get_screen_width() as f32, d.get_screen_height() as f32 );
        }

        gui.layout(&screen_size);
        d.clear_background(Color::WHITE);

        gui.render(&mut d, Vector2{x:0.0,y:0.0});

        // d.draw_line(0, 120, screen_size.width as i32, 120, Color::RED);
        // d.draw_line(120, 0, 120, screen_size.height as i32, Color::RED);
    }
}
