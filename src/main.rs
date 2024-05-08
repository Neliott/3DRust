use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use crate::game::transform::Transform;
use crate::math_structs::vector3::Vector3;
use crate::math_structs::vector3int::Vector3Int;
use crate::rendering::camera::Camera;
use crate::rendering::mesh::Mesh;
use crate::rendering::mesh_renderer::MeshRenderer;
use crate::rendering::renderer::Renderer;

mod game;
mod math_structs;
mod rendering;
mod color;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        //.borderless()
        .resizable()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");



    let cube_vertices: Vec<Vector3> = vec![
        Vector3 { x: -1f32, y: -1f32, z: -1f32 },
        Vector3 { x: 1f32, y: -1f32, z: -1f32 },
        Vector3 { x: 1f32, y: 1f32, z: -1f32 },
        Vector3 { x: -1f32, y: 1f32, z: -1f32 },
        Vector3 { x: -1f32, y: -1f32, z: 1f32 },
        Vector3 { x: 1f32, y: -1f32, z: 1f32 },
        Vector3 { x: 1f32, y: 1f32, z: 1f32 },
        Vector3 { x: -1f32, y: 1f32, z: 1f32 }
    ];
    let cube_indices: Vec<Vector3Int> = vec![
        Vector3Int { x: 0, y: 1, z: 2 },
        Vector3Int { x: 0, y: 2, z: 3 },
        Vector3Int { x: 1, y: 5, z: 6 },
        Vector3Int { x: 1, y: 6, z: 2 },
        Vector3Int { x: 5, y: 4, z: 7 },
        Vector3Int { x: 5, y: 7, z: 6 },
        Vector3Int { x: 4, y: 0, z: 3 },
        Vector3Int { x: 4, y: 3, z: 7 },
        Vector3Int { x: 3, y: 2, z: 6 },
        Vector3Int { x: 3, y: 6, z: 7 },
        Vector3Int { x: 4, y: 5, z: 1 },
        Vector3Int { x: 4, y: 1, z: 0 }
    ];
    let transform = Transform{
        position: Vector3::zero(),
        rotation: Vector3::zero(),
        scale: Vector3::one()
    };
    let mesh = Mesh::new(cube_vertices, cube_indices);
    let cube = MeshRenderer::new(transform, mesh);
    let mut camera = Camera::new(Transform{
        position: Vector3::new(0f32, 0f32, -10f32),
        rotation: Vector3::zero(),
        scale: Vector3::one()
    });
    camera.fov = 180f32;

    let mut render = Renderer::new(camera);
    render.add_mesh_renderer(cube);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    render.camera.transform.position.z += 0.1f32;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    render.camera.transform.position.z -= 0.1f32;
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    render.camera.transform.position.x += 0.1f32;
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    render.camera.transform.position.x -= 0.1f32;
                },
                Event::MouseMotion { xrel, yrel, .. } => {
                    render.camera.transform.rotation.y += xrel as f32;
                    render.camera.transform.rotation.x += yrel as f32;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        render.render(&mut canvas);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }


    Ok(())
}