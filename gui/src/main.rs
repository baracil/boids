use raylib::prelude::*;

use gui::gui::Gui;


use gui::widget_operation::{WidgetOp, WidgetDataProvider};

use gui::alignment::VAlignment::{Bottom, Top, Center};
use gui::alignment::HAlignment::{Left, Right, Middle};

use gui::widget::Widget::{Pane, VBox, Label};
use gui::pane::PanePar;
use gui::size::Size;
use gui::background::Background::Solid;
use gui::border::Border::Line;
use gui::position::Coordinate::{Relative, Absolute};
use gui::vbox::VBoxPar;
use gui::label::LabelPar;
use std::panic::panic_any;
use gui::padding::Padding;
use gui::fill::Fill::Enabled;
use std::f32::consts::PI;
use gui::fill::Fill;


fn main() {

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .vsync()
        .resizable()
        .msaa_4x()
        .title("Gui Test")
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
    gui.add_background("default",Solid {idle_color:Color::DARKBLUE, hoovered_color:Color::SKYBLUE, armed_color:Color::BLUE});
    gui.add_background("red",Solid {idle_color:Color::RED,hoovered_color:Color::ORANGE, armed_color:Color::LIME});
    gui.add_background("yellow",Solid {idle_color:Color::YELLOW,hoovered_color:Color::YELLOW, armed_color:Color::YELLOW});

    let root_pane = {
        let par = PanePar::new();
        par.set_preferred_height(&gui, 200.0)
            .set_preferred_width(&gui, 600.0)
            .set_padding(&gui,Padding::same(10.0))
            .set_position(&gui, &Absolute(0.0),&Absolute(0.0))
            .set_valignment(&gui, Top)
            .set_halignment(&gui, Left)
            .enable_fill_width(&gui, Enabled {weight:1})
            .enable_fill_height(&gui, Enabled {weight:1} )
            .set_action_id("ROOT");
        gui.insert_root(Pane(par))
    };

    let _vbox = {
        let par = VBoxPar::new();
        par.set_spacing(&gui,20.0)
            .set_position(&gui,&Relative(50.0), &Relative(50.0))
            .set_halignment(&gui, Left)
            .set_valignment(&gui,Top)
            .set_padding(&gui, Padding::same(10.0))
            .set_background_style("yellow");
        gui.add_child(root_pane,VBox(par))
    };


    let padding = Padding::new(15.0,20.0,10.0,5.0);

    let _vbox1 = {
        let _vbox1 = {
            let par = VBoxPar::new();
            par.set_spacing(&gui, 10.0)
                .set_background_style("red")
                .set_padding(&gui, padding)
                .set_position(&gui, &Relative(50.0), &Relative(50.0))
                .set_valignment(&gui, Center)
                .set_halignment(&gui, Middle)
                .set_action_id("VBOX");
            gui.add_child(_vbox, VBox(par))
        };

        let _label1 = {
            let par = LabelPar::new();
            par.set_text(&gui, "Label 1")
                .set_action_id("Label1")
                .set_clickable(true)
                .set_padding(&gui, Padding::new(0.0, 5.0, 0.0, 5.0));
            gui.add_child(_vbox1, Label(par))
        };

        let _label2 = {
            let par = LabelPar::new();
            par.set_text(&gui, "Long label with several words")
                .set_action_id("Label2")
                .set_clickable(true)
                .set_padding(&gui, Padding::new(0.0, 5.0, 0.0, 5.0));
            gui.add_child(_vbox1, Label(par))
        };

        let _label3 = {
            let par = LabelPar::new();
            par.set_text(&gui, "3")
                .set_padding(&gui, Padding::new(0.0, 5.0, 0.0, 5.0))
                .set_action_id("Label3")
                .set_clickable(true)
                .enable_fill_width(&gui, Enabled { weight: 1 })
            ;

            gui.add_child(_vbox1, Label(par))
        };
        _vbox1
    };

    let _vbox2 = {
        let _vbox2 = {
            let par = VBoxPar::new();
            par.set_spacing(&gui, 10.0)
                .set_background_style("red")
                .set_padding(&gui, padding)
                .set_position(&gui, &Relative(50.0), &Relative(50.0))
                .set_valignment(&gui, Center)
                .set_halignment(&gui, Middle)
                .set_action_id("VBOX");
            gui.add_child(_vbox, VBox(par))
        };

        let _label1 = {
            let par = LabelPar::new();
            par.set_text(&gui, "Label 1")
                .set_action_id("Label1 b")
                .set_clickable(true)
                .set_padding(&gui, Padding::new(0.0, 5.0, 0.0, 5.0));
            gui.add_child(_vbox2, Label(par))
        };

        let _label2 = {
            let par = LabelPar::new();
            par.set_text(&gui, "Long label with several words")
                .set_action_id("Label2 b")
                .set_clickable(true)
                .set_padding(&gui, Padding::new(0.0, 0.0, 0.0, 15.0));
            gui.add_child(_vbox2, Label(par))
        };

        let _label3 = {
            let par = LabelPar::new();
            par.set_text(&gui, "3")
                .set_padding(&gui, Padding::new(0.0, 15.0, 0.0, 0.0))
                .set_action_id("Label3 b")
                .set_clickable(true)
                .enable_fill_width(&gui, Enabled { weight: 1 })
            ;

            gui.add_child(_vbox2, Label(par))
        };
        _vbox2
    };



    let mut screen_size: Size = Size::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

    let offset = Vector2{x:0.0, y:0.0};

    let vbox = gui.get_widget(_vbox).unwrap();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if d.is_window_resized() {
            screen_size = Size::new(d.get_screen_width() as f32, d.get_screen_height() as f32 );
        }

        let time = d.get_time() as f32;

        vbox.widget_data().set_position(&gui,&Absolute(time*20.), &Absolute(time*20.));



        gui.update_states(&d, &offset);
        gui.layout(&screen_size);

        d.clear_background(Color::WHITE);

        gui.render(&mut d, &offset);

        gui.display_events();
    }
}
