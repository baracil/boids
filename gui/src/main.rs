use raylib::prelude::*;

use gui::gui::Gui;


use gui::widget_operation::{WidgetOp};

use gui::alignment::VAlignment::{Bottom, Top, Center};
use gui::alignment::HAlignment::{Left, Right, Middle};

use gui::widget::Widget::Pane;
use gui::pane::PanePar;
use gui::size::Size;
use gui::background::Background::Solid;
use gui::border::Border::Line;


fn main() {

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .vsync()
        .resizable()
        .msaa_4x()
        .title("Hello, World")
        .build();

    let mut gui = Gui::new();
    gui.load_font(&mut rl, &thread,
                  "default",
                  "/home/Bastien Aracil/Downloads/FreckleFace-Regular.ttf",
                  48,
                  200,
    ).expect("Could not load the font");

    gui.add_text_style("default","default",Color::BLACK,0.0);
    gui.add_border("default",Line {color:Color::BLACK, thickness:2.0});
    gui.add_background("default",Solid {color:Color::SKYBLUE});
    gui.add_background("red",Solid {color:Color::RED});

    let root_pane = {
        let par = PanePar::new();
        par.set_preferred_height(&gui, 200.0)
            .set_preferred_width(&gui, 400.0)
            .set_absolute_coordinate_x(&gui, false)
            .set_absolute_coordinate_y(&gui, false)
            .set_position(&gui, 50.0, 50.0)
            .set_valignment(&gui, Center)
            .set_halignment(&gui, Middle);
        gui.insert_root(Pane(par))
    };


    let child_pane = {
        let par = PanePar::new();
        par.set_preferred_height(&gui, 20.0)
            .set_preferred_width(&gui, 20.0)
            .set_absolute_coordinate_x(&gui, false)
            .set_absolute_coordinate_y(&gui, false)
            .set_background_style("red")
            .set_position(&gui, 50.0, 100.0)
            .set_valignment(&gui, Bottom)
            .set_halignment(&gui, Middle);
        gui.add_child(root_pane, Pane(par))
    };



    let mut screen_size: Size = Size::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

    let offset = Vector2{x:0.0, y:0.0};

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if d.is_window_resized() {
            screen_size = Size::new(d.get_screen_width() as f32, d.get_screen_height() as f32 );
        }

        gui.layout(&screen_size);
        d.clear_background(Color::WHITE);

        gui.render(&mut d, &offset, &screen_size);

    }
}
