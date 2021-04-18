use raylib::prelude::*;

use gui::alignment::Alignment;
use gui::alignment::HAlignment::{Left, Middle, Right};
use gui::alignment::VAlignment::{Bottom, Center, Top};
use gui::gui::create_gui;
use gui::gui::Gui;
use gui::label::LabelPar;
use gui::widget::Widget::Label;
use gui::widget_operation::{LayoutableWidget, RenderableWidget, WidgetOp};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .vsync()
        .resizable()
        .msaa_4x()
        .title("Hello, World")
        .build();

    // let gui : Gui;
    //

    //
    // let menu = gui.create_menu();
    //
    // gui.create_button(gui.create_label(""), gui.create_icon());
    //
    // menu.add_item(gui.create_button("Level Editor", font_id));
    // menu.add_item(gui.create_button("Level Editor", font_id));
    //
    //
    // let grid = gui.create_grid(nb_rows, nb_cols);
    // grid.add_item(item,0,0);
    //
    // let label = gui.create_label("Hello",font_id);
    //
    //    let font = rl.get_font_default();
    //

    //        let font = rl.load_font_ex(&thread,"/home/Bastien Aracil/Downloads/FreckleFace-Regular.ttf",48,FontLoadEx::Default(200)).unwrap();
    //    let font = rl.load_font_ex(&thread,"/home/Bastien Aracil/Downloads/pixantiqua.ttf",48,FontLoadEx::Default(80)).unwrap();

    let mut gui = create_gui();

    let id = gui
        .load_font(
            &mut rl,
            &thread,
            "/home/Bastien Aracil/Downloads/FreckleFace-Regular.ttf",
            48,
            200,
        )
        .unwrap();
    let font_info = gui.get_font(id.as_str()).unwrap();

    let label = Label(LabelPar::new(font_info));

    // let mut label = Label::new(font_info);
    // label.set_padding(0.0);
    // label.set_text(String::from("Hello"));
    // label.set_position(
    //     &Vector2 { x: 120.0, y: 120.0 },
    //     Alignment {
    //         vertical: Bottom,
    //         horizontal: Left,
    //     },
    // );
    //
    // label.layout();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let screen_width = d.get_screen_width();
        let screen_height = d.get_screen_height();

        d.clear_background(Color::WHITE);

        // label.render(&mut d);

        d.draw_line(0, 120, screen_width, 120, Color::RED);
        d.draw_line(120, 0, 120, screen_height, Color::RED);
    }
}
