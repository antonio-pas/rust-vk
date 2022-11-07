use ash::{vk, Entry};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::ffi::CString;
fn utf8_to_string(buffer: [u8; 256]) -> String {
    let mut string = String::from_utf8(buffer.to_vec()).unwrap();
    string.retain(|c| c != '\0');
    string
}
fn main() {
    let sdl = sdl2::init().expect("error initializing SDL2!");
    let video = sdl
        .video()
        .expect("error initializing SDL2 video subsystem!");
    let window = video
        .window("Hello", 1280, 720)
        .position_centered()
        .vulkan()
        .build()
        .expect("error creating SDL2 window");

    let entry = Entry::linked();

    // getting extensions
    let available_exts: Vec<String> = entry
        .enumerate_instance_extension_properties(None)
        .expect("error enumerating Vulkan extension properties")
        .iter()
        .map(|ext| {
            utf8_to_string(ext.extension_name)
        })
        .collect();
    let required_exts = window
        .vulkan_instance_extensions()
        .expect("error querying SDL Vulkan extensions!");

    println!("Available extensions:");
    for ext in &available_exts {
        println!("\t{}", ext);
    }
    println!("Required extensions:");
    for ext in &required_exts {
        println!("\t{}", ext);
    }
    let required_exts: Vec<_> = required_exts
        .iter()
        .map(|ext| {
            let cstr = CString::new(ext.clone()).unwrap();
            cstr.into_raw() as *const u8
        })
        .collect();

    // getting validation layers
    let layers = entry.enumerate_instance_layer_properties().unwrap();
    for layer in layers {
        println!("{}", utf8_to_string(layer.layer_name));
    }

    // creating vulkan instance
    let app_info = vk::ApplicationInfo {
        api_version: vk::make_api_version(0, 1, 0, 0),
        ..Default::default()
    };
    let instance_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        enabled_extension_count: required_exts.len() as u32,
        pp_enabled_extension_names: required_exts.as_ptr(),
        ..Default::default()
    };
    let instance = unsafe {
        entry
            .create_instance(&instance_info, None)
            .expect("failed to create Vulkan instance")
    };

    // main loop
    let mut event_pump = sdl.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }
    unsafe {
        instance.destroy_instance(None);
    }
}
