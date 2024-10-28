#![allow(
    dead_code,
    unused_variables,
    clippy::manual_slice_size_calculation,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::usize;
use app::{App, AppData};
use std::ptr::copy_nonoverlapping as memcpy;
use anyhow::Result;
use cgmath::{vec2, vec3};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::Version;
use winit::dpi::LogicalSize;
use winit::event::{Event, MouseButton, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use crate::structs::Vertex;
use winit::keyboard::{Key, PhysicalKey};

pub mod logfile;
pub mod proc;
pub mod mtr;
pub mod hexadump;
pub mod welcome;
pub mod scene;
pub mod instance;
pub mod app;
pub mod physicalDevice;
pub mod logicalDevice;
pub mod swapchain;
pub mod pipeline;
pub mod frameBuffers;
pub mod commandPool;
pub mod texture;
pub mod buffers;
pub mod descriptors;
pub mod sharedBuffer;
pub mod commandBuffers;
pub mod syncObjects;
pub mod sharedImages;
pub mod sharedOther;
pub mod structs;
pub mod rustDoom;

use rustDoom::RustDoom;

/// Whether the validation layers should be enabled.
const VALIDATION_ENABLED: bool = false /*cfg!(debug_assertions)*/;
/// The name of the validation layers.
const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

/// The required device extensions.
const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];
/// The Vulkan SDK version that started requiring the portability subset extension for macOS.
const PORTABILITY_MACOS_VERSION: Version = Version::new(1, 3, 216);

/// The maximum number of frames that can be processed concurrently.
const MAX_FRAMES_IN_FLIGHT: usize = 2;




#[rustfmt::skip]
static VERTICES: [Vertex; 4] = [
    Vertex::new(vec2(-0.5, -0.5), vec3(1.0, 0.0, 0.0), vec2(1.0, 0.0)),
    Vertex::new(vec2(0.5, -0.5), vec3(0.0, 1.0, 0.0), vec2(0.0, 0.0)),
    Vertex::new(vec2(0.5, 0.5), vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0)),
    Vertex::new(vec2(-0.5, 0.5), vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

#[rustfmt::skip]
fn main() -> Result<()> {
    pretty_env_logger::init();

    let ourRustDoom = RustDoom::createRustDoom();

    // Window

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("rustDoom")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    // App

    let mut app = unsafe { App::create(&window)? };
    let mut minimized = false;
    event_loop.run(move |event, elwt| {
        match event 
        {
            // Request a redraw when all events were processed.
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, .. } => match event 
            {
                // Render a frame if our Vulkan app is not being destroyed.
                WindowEvent::RedrawRequested if !elwt.exiting() && !minimized => {
                    unsafe { app.render(&window) }.unwrap();
                },
                // Mark the window as having been resized.
                WindowEvent::Resized(size) => {
                    if size.width == 0 || size.height == 0 {
                        minimized = true;
                    } else {
                        minimized = false;
                        app.resized = true;
                    }
                }
                
                WindowEvent::KeyboardInput { event, is_synthetic:false, ..} =>
                {
                    //let mods = window.modifiers;

                    // Dispatch actions only on press.
                    if event.state.is_pressed() 
                    {
                        println!("physical_key: {:?}", event.physical_key);
                        if event.physical_key == winit::keyboard::KeyCode::Escape
                        {
                            println!("ESC"); 
                            elwt.exit();
                            unsafe { app.destroy(); }                                   
                        }

                        if let Key::Character(ch) = event.logical_key.as_ref() 
                        {
                            println!("logical_key: {:?}", event.logical_key);
                            println!("{}", ch.to_uppercase());    
                            //if ch==27
                            //{
                            //    print!("ESC");                                    
                            //}
                        } 
    
                    }

                }

                WindowEvent::MouseInput { device_id, state, button } =>
                {
                    //println!("WindowEvent::MouseInput {:?}", event);
                    if button == MouseButton::Left      {println!("Left   Mouse Button Pressed");};
                    if button == MouseButton::Middle    {println!("Middle Mouse Button Pressed");};
                    if button == MouseButton::Right     {println!("Right  Mouse Button Pressed");};

                }

                // Destroy our Vulkan app.
                WindowEvent::CloseRequested => 
                {
                    elwt.exit();
                    unsafe { app.destroy(); }
                }

                WindowEvent::CursorMoved { device_id, position } =>
                {
                    //println!("WindowEvent::CursorMoved {:?}", event);
                    println!("{:?}", position);
                }

                _ => 
                {
                    //println!("{:?}", event);
                }
            }
            _ => {}
        }
    })?;

    Ok(())
}
