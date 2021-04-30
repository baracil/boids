use raylib::prelude::*;

use crate::data::boid::Boid;
use crate::data::world::World;
use gui::gui::{Gui, GuiData};
use gui::widget::Widget::{VBox, Label, Slider};
use gui::vbox::VBoxPar;
use gui::padding::Padding;
use gui::border::Border;
use gui::border::Border::Line;
use gui::background::Background;
use gui::background::Background::Solid;
use gui::mouse::MouseState;
use gui::size::Size;
use gui::fill::Fill;
use gui::fill::Fill::Enabled;
use gui::alignment::VAlignment::Top;
use gui::alignment::HAlignment::Left;
use gui::position::Coordinate::Absolute;
use gui::slider::SliderPar;
use std::process::abort;
use gui::event::Event::Drag;

mod data;

const DEFAULT_NB_BIRDS: usize = 2000;
const DEFAULT_WORLD_SIZE: f32 = 10.;

pub struct ScreenSize {
    pub width: i32,
    pub height: i32,
}

const COHESION_ID:&str = "cohesion_id";
const SEPARATION_ID:&str = "separation_id";
const ALIGNMENT_ID:&str = "alignment_id";

fn draw_birds(d: &mut impl RaylibDraw, boids: &[Boid], bird_size: f32) {
    {
        let size_factor: f32 = 1.2;

        let mut head = Vector2 { x: 0.0, y: 0.0 };
        let mut left_wing = Vector2 { x: 0.0, y: 0.0 };
        let mut right_wing = Vector2 { x: 0.0, y: 0.0 };

        for boid in boids {
            let nvx = size_factor * bird_size * boid.velocity.x / boid.speed;
            let nvy = size_factor * bird_size * boid.velocity.y / boid.speed;
            head.x = nvx + boid.position.x;
            head.y = nvy + boid.position.y;

            left_wing.x = nvy * 0.3 + boid.position.x;
            left_wing.y = -nvx * 0.3 + boid.position.y;

            right_wing.x = -nvy * 0.3 + boid.position.x;
            right_wing.y = nvx * 0.3 + boid.position.y;

            d.draw_triangle(head, left_wing, right_wing, Color::BLACK);
        }
    }
}

pub struct BoidsModel {
    pub gui_width: f32,
    pub screen_size: ScreenSize,
    pub world: World,
}

impl BoidsModel {
    pub fn new(nb_birds: usize, world_size: f32) -> Self {
        BoidsModel {
            gui_width: 200.0,
            screen_size: ScreenSize {
                width: 0,
                height: 0,
            },
            world: World::new(nb_birds, world_size),
        }
    }
}

impl BoidsModel {
    pub fn camera_offset(&self) -> Vector2 {
        return Vector2 {
            x: (self.screen_size.width as f32 + self.gui_width) * 0.5,
            y: self.screen_size.height as f32 * 0.5,
        };
    }
    pub fn camera_zoom(&self) -> f32 {
        return 0.8 * self.screen_size.width.min(self.screen_size.height) as f32
            / (self.world.playfield_size * 2.0);
    }
}

fn main() {
    let mut app_state = BoidsModel::new(DEFAULT_NB_BIRDS, DEFAULT_WORLD_SIZE);

    app_state.world.initialize();


    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .msaa_4x()
        .resizable()
        .vsync()
        .title("Boids")
        .build();

    rl.set_target_fps(60);


    let mut gui = Gui::new();

    gui.load_font(&mut rl, &thread, "default", "/home/Bastien Aracil/Downloads/FreckleFace-Regular.ttf", 48, 200);
    gui.load_font(&mut rl, &thread, "small", "/home/Bastien Aracil/Downloads/FreckleFace-Regular.ttf", 20, 200);

    let red = Color::RED;

    gui.add_border("default", Line { color: Color::BLACK, thickness: 1.0 });
    gui.add_background("red", Solid { idle_color: red, hoovered_color: red, armed_color: red });
    gui.add_text_style("default","default",Color::BLACK,0.0);

    let container = {
        let par = VBoxPar::new();
        par.set_spacing(&gui, 50.)
            .set_padding(&gui, Padding::same(20.0))
            .set_border_style("default")
            .set_preferred_width(&gui, 300.0)
            .enable_fill_height(&gui, Enabled { weight: 1 })
            .set_position(&gui, &Absolute(0.0), &Absolute(0.0))
            .set_alignment(&gui, Top, Left)
            .set_background_style("red");
        gui.insert_root(VBox(par))
    };

    {
        let par = SliderPar::new();
        par.set_value(&gui, 100.0 * app_state.world.parameters.alignment_factor)
            .set_value_min(&gui, 0.0)
            .set_value_max(&gui, 100.0)
            .set_text_style("default")
            .set_action_id(ALIGNMENT_ID)
            .set_text_style("default")
            .enable_fill_width(&gui, Enabled { weight: 1 });

        gui.add_child(container, Slider(par));
    }

    {
        let par = SliderPar::new();
        par.set_value(&gui, 100.0 * app_state.world.parameters.cohesion_factor)
            .set_value_min(&gui, 0.0)
            .set_value_max(&gui, 100.0)
            .set_text_style("default")
            .set_action_id(COHESION_ID)
            .set_text_style("default")
            .enable_fill_width(&gui, Enabled { weight: 1 });

        gui.add_child(container, Slider(par));
    }
    {
        let par = SliderPar::new();
        par.set_value(&gui, 100.0 * app_state.world.parameters.separation_factor)
            .set_value_min(&gui, 0.0)
            .set_value_max(&gui, 100.0)
            .set_text_style("default")
            .set_action_id(SEPARATION_ID)
            .set_text_style("default")
            .enable_fill_width(&gui, Enabled { weight: 1 });

        gui.add_child(container, Slider(par));
    }


    let mut camera = Camera2D {
        target: Vector2 { x: 0., y: 0. },
        offset: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
        zoom: 1.0,
    };


    app_state.screen_size.width = rl.get_screen_width();
    app_state.screen_size.height = rl.get_screen_height();

    let mut mouse_state = MouseState::new();
    let mut screen_size: Size = Size::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);
    let mut should_quit = false;
    let offset = Vector2::zero();
    while !rl.window_should_close() && !should_quit {
        let mut d = rl.begin_drawing(&thread);

        if d.is_window_resized() {
            screen_size = Size::new(d.get_screen_width() as f32, d.get_screen_height() as f32);
            app_state.screen_size.width = d.get_screen_width();
            app_state.screen_size.height = d.get_screen_height();
            camera.offset = app_state.camera_offset();
            camera.target.x = 0.0;
            camera.target.y = 0.0;
            camera.zoom = app_state.camera_zoom();
        }

        d.clear_background(Color::WHITE);
        d.draw_fps(app_state.screen_size.width - 100, 0);

        mouse_state.update(&d);


        gui.layout_and_render(&mut d, &screen_size, &mouse_state, &offset);


        {
            let mut d = d.begin_mode2D(camera);
            draw_birds(
                &mut d,
                &(app_state.world.current[..]),
                app_state.world.parameters.bird_size,
            );
        }

        let dt = d.get_frame_time();

        let events = gui.get_events();
        for event in events.iter() {
            if let Drag(p) = event {
                match p.action_id() {
                    COHESION_ID => {app_state.world.parameters.cohesion_factor = p.value()/100.},
                    ALIGNMENT_ID => {app_state.world.parameters.alignment_factor = p.value()/100.},
                    SEPARATION_ID => {app_state.world.parameters.separation_factor = p.value()/100.},
                    &_ => {}
                }
            }

        }

        app_state.world.compute(dt);
    }
}
