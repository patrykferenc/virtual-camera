mod projection;
mod scene;

use cgmath::{perspective, vec4};
use scene::polygon::Polygon;
use scene::vertex::Vertex;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub fn main() -> Result<(), String> {
    let mut scene = scene::scene::Scene::new();

    // TODO: read scene from file
    let v1 = Vertex::new(0.0, 10.0, 0.0);
    let v2 = Vertex::new(-10.0, 0.0, -10.0);
    let v3 = Vertex::new(10.0, 0.0, -10.0);
    let v4 = Vertex::new(10.0, 0.0, 10.0);
    let v5 = Vertex::new(-10.0, 0.0, 10.0);

    let polygons = vec![
        Polygon::new(v1, v2, v3),
        Polygon::new(v1, v3, v4),
        Polygon::new(v1, v4, v5),
        Polygon::new(v1, v5, v2),
        Polygon::new(v5, v4, v3),
        Polygon::new(v5, v3, v2),
    ];

    for polygon in polygons {
        scene.add_polygon(polygon);
    }

    let global_state_vector = vec4(0., 0., -10., 0.);
    let mut camera = projection::camera::Camera::new(600.0, 800.0, 1., global_state_vector);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Virtual camera with rust-sdl2 demo: Window", 800, 600)
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut tick = 0;

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => camera.rotate_x(5.0),
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => camera.rotate_x(-5.0),
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => camera.rotate_y(-5.0),
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => camera.rotate_y(5.0),

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => camera.translate_left(),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => camera.translate_right(),
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => camera.translate_forward(),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => camera.translate_backward(),
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => camera.change_zoom(-0.1),
                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => camera.change_zoom(0.1),
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => camera.reset_zoom(),
                _ => {}
            }
        }

        {
            // Update the window title.
            let window = canvas.window_mut();

            let position = window.position();
            let size = window.size();
            let title = format!(
                "Window - pos({}x{}), size({}x{}): {}",
                position.0, position.1, size.0, size.1, tick
            );
            window.set_title(&title).map_err(|e| e.to_string())?;

            tick += 1;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw the scene.
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        for polygon in scene.polygons() {
            let vertices = polygon.vertices();
            let v1 = vertices[0];
            let v2 = vertices[1];
            let v3 = vertices[2];

            let projected_v1 = camera.project(v1);
            let projected_v2 = camera.project(v2);
            let projected_v3 = camera.project(v3);

            // TODO Make it civilised xd
            if projected_v2.is_ok() && projected_v3.is_ok() {
                canvas.draw_line(camera.project(v2)?, camera.project(v3)?)?;
            }
            if projected_v2.is_ok() && projected_v1.is_ok() {
                canvas.draw_line(camera.project(v2)?, camera.project(v1)?)?;
            }
            if projected_v3.is_ok() && projected_v1.is_ok() {
                canvas.draw_line(camera.project(v3)?, camera.project(v1)?)?;
            }
        }
        canvas.present();
    }

    Ok(())
}
