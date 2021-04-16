use gui::node::Item::Label;
use gui::label::LabelPar;
use raylib::prelude::*;
use gui::node::{FontInfo, NodePar};
use gui::private_node::get_node_trait_mut;
use gui::alignment::Alignment;
use gui::alignment::VAlignment::{Top, Center, Bottom};
use gui::alignment::HAlignment::{Left, Middle, Right};
use gui::gui::Gui;
use gui::gui::create_gui;


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
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


    let mut gui  = create_gui();


    let id = gui.load_font(&mut rl,&thread,"/home/Bastien Aracil/Downloads/FreckleFace-Regular.ttf",48,200).unwrap();
    let font_info = gui.get_font(id.as_str()).unwrap();

    let mut par = LabelPar::new(font_info);
    par.set_padding(10.0);
    par.text = Some(String::from("Hello2"));
    par.set_position(&Vector2{x:120.0,y:120.0},Alignment{ vertical:Bottom, horizontal:Right});
    let mut label = Label(par);

    get_node_trait_mut(&mut label).layout();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let screen_width = d.get_screen_width();
        let screen_height = d.get_screen_height();

        d.clear_background(Color::WHITE);

        get_node_trait_mut(&mut label).render(&mut d);

        d.draw_line(0,120,screen_width,120, Color::RED);
        d.draw_line(120,0,120,screen_height, Color::RED);

    }
}
