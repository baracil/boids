mod data;
mod gui_form;

use raylib::prelude::*;

use crate::data::boid::Boid;
use crate::data::world::World;
use crate::gui_form::{ButtonPar, GuiElement, get_gui_item};
use crate::gui_form::GuiElement::Button;

const DEFAULT_NB_BIRDS: usize = 500;
const DEFAULT_WORLD_SIZE: f32 = 10.;


pub struct ScreenSize {
    pub width: i32,
    pub height: i32,
}

pub struct Gui {
    pub quit_button: GuiElement,
}

fn draw_gui(d: &mut RaylibDrawHandle, app_state: &mut BoidsModel) -> bool {
    d.draw_rectangle(0, 0, app_state.gui_width as i32, app_state.screen_size.height, Color::RED);


    {
        let mut should_quit = false;

        should_quit|= draw_element(d,&mut app_state.gui.quit_button,&Vector2{x:0.0,y:0.0});
        should_quit|= draw_element(d,&mut app_state.gui.quit_button,&Vector2{x:0.0,y:60.0});

        return should_quit
    }

}


fn draw_element(d: &mut RaylibDrawHandle, element:&mut GuiElement, position:&Vector2) -> bool {
    let mouse_position = d.get_mouse_position();
    let left_pressed = d.is_mouse_button_released(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON);

    let gui_item = get_gui_item(element);

    gui_item.set_position(position);

    d.draw_rectangle_rec(gui_item.geometry(), gui_item.background_color());

    let inside = gui_item.geometry().check_collision_point_rec(mouse_position);

    if left_pressed && inside {
        let element_id: &str = {
            match &element {
                Button(a) => { a.id.as_str() }
                GuiElement::Slider(a) => { a.id.as_str() }
            }
        };

        return on_button_pressed(element_id);
    }
    false
}



fn on_button_pressed( id: &str) -> bool {
    if id == "quit" {
        println!("QUIT PRESSED");
        true
    } else {
        false
    }
}

fn draw_birds<'a>(d: &mut RaylibDrawHandle<'a>,
                  camera: &Camera2D,
                  boids: &[Boid],
                  bird_size: f32) {
    {
        let size_factor: f32 = 1.2;
        let mut d = d.begin_mode2D(camera);

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
    pub gui: Gui,
    pub world: World,
}

impl BoidsModel {
    pub fn new(nb_birds: usize, world_size: f32) -> Self {
        let quit_button = ButtonPar {
            id: "quit".to_string(),
            geometry: Rectangle{width:100.,height:30.,x:0.0,y:0.0},
            color: Color::BLACK,
            background_color: Color::GREEN,
            text: "Quit".to_string(),
        };
        BoidsModel {
            gui_width: 200.0,
            screen_size: ScreenSize { width: 0, height: 0 },
            gui: Gui {
                quit_button: Button(Box::new(quit_button))
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
        return 0.8 * self.screen_size.width.min(self.screen_size.height) as f32 / (self.world.playfield_size * 2.0);
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


    let mut camera_outdated = true;
    let mut camera = Camera2D { target: Vector2 { x: 0., y: 0. }, offset: Vector2 { x: 0.0, y: 0.0 }, rotation: 0.0, zoom: 1.0 };

    let mut should_quit = false;
    while !rl.window_should_close() && !should_quit {
        let mut d = rl.begin_drawing(&thread);

        camera_outdated = camera_outdated || d.is_window_resized();

        if camera_outdated {
            app_state.screen_size.width = d.get_screen_width();
            app_state.screen_size.height = d.get_screen_height();
            camera.offset = app_state.camera_offset();
            camera.target.x = 0.0;
            camera.target.y = 0.0;
            camera.zoom = app_state.camera_zoom();
        }

        d.clear_background(Color::WHITE);
        d.draw_fps( app_state.screen_size.width-100,0);

        {
            should_quit |= draw_gui(&mut d, &mut app_state);
            draw_birds(&mut d, &mut camera, &(app_state.world.current[..]), app_state.world.parameters.bird_size);
        }

        let dt = d.get_frame_time();

        app_state.world.compute(dt);
    }
}

