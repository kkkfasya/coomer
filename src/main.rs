/*..........................................................................................
..........................................................................................
.........................................,:...,,..........................................
.........................................,+;..+;::,:......................................
...........................,,......,,:++;;??**%?????+::,..................................
...........................,+;:*++*+;;:;*?**?%%**%??++****;::,............................
...........................:?%%%?+,,....,:+*?%*?%?+,::.:*????;............................
.........................,+*,:::,.........,::++*;:......,;;;;*+,..........................
........................;*;..................................,;*;.........................
........................?:.......,,,,:,::::;:,,,,,,,,,,,,......:?.........................
.......................+*......:::::::;:::,,::::::::::::,......,*+........................
......................+*......,:::::::::::::;::::::::::::,.......?;.......................
......................%:............,,,::::::::::,,,,,,,,,,......;?.......................
.....................:?.......,:::::;::,,,,...,,:::::::::::,.....,%:......................
.....................%:.......,,,,,,,,,............,::::::,,......;*......................
....................,%,....,:::::::::::..........,,::,,,,:::;:,...:?......................
....................:?.....,,.........,;,.......,;:........,,:,...,?,.....................
....................:*.....:+;;;::,,...,;.......,,.,::;+?%?*%:..,..?:.....................
....................;*.....,?*?@@@#%*+:.,,,....,..**+%@@@@S;%:.....?:.....................
....................,?.....::+*#SSS?*%*.,,,,...,,,*%*%S###%+%:....,%,.....................
.....................*+.,..:+?%?????*:.,,,,,...,.,,;?***++??+,...,?;......................
......................?+*,,..,:,,::::,.,.;;.....+,.,:;++;:::,..:;+?.......................
.....................,?+**+..........,,,;?:..,,.++............,*%?%,......................
.....................,%?++;:,.....,,:;+?*:......,+*:,,,,.,,,..,?;%S:......................
.....................:%%*%*?:..,,::::;+,..........,+*,:;;,,,,.+*??%;......................
.....................;%%???*,.,:;:,,.,*:,,..,,....,*+.,,:+:,,,*+%%*,......................
.....................:?%????*+;+;;+*;++*+;+;::;;+;+*;;++,;;;;;?%??;,......................
......................:?%%%%?+*??%?*%;;+;;;+;;++++++?%***??*?%%%?**,......................
.......................:%%??+?*???*+?+:,....,,,,.,,:++;;:+%+*??%*+:.......................
.......................;??%%%%??%????;+*;:,,,,,,:::;:,+????%??%?:,........................
.......................::,:%%??*??%?**?*;:::;:;;:::,:+?%%%?%%%%+,.........................
..........................,?*??????+%%*??+*::;;*;;?+%?%%?%?%%%??;.........................
..........................*;,**?%%???*?+*;%?+++*%?%%%??%%%?%?*+,,.........................
.........................:?...,,*%%??+**%%%??%**%*%%%*?%%?,;,.?:..........................
........................,*+.....::,+?%*??%?++%?????+?%%?+:....++..........................
.......................,?+......,::.:+++***?%?%**??**:;;,.....,**.........................
......................:?;.....,+:,,.......,,::+,,;:...;,........*+........................*/

/* (*Wakes up) The birds are singing, the flowers are blooming, my dick is throbbing, 
 * what a beautiful day for cooming. Good morning Alix Lynx, I've been awake for 20 whole seconds 
 * and I haven't coomed yet. It's time to hope on my porn throne and machine gun jackhammer 
 * my bloodshot death-grip bloodshot semi chub with my roided doomfist once again! 
 * (Types on keyboard). I-s...is that a Female Girl?? HMMGH, I-I MUST SNIFF, SNNNNNNNNIIIIIIIIIIIIFFFFFFFF** 
 * OH GOD (FAPFAPFAPFAP) FUCCKK, HUHGHU, SNIIFF, HUHGJGUHHGUGHU (SMASHES DESK) I-I-IM COOOMING!!!!! IM COOMING, 
 * IM COOMING IM COOOMING IM COOOOOMING COOOOOOOOOOM, COOOMING, FUCCKKK, AHHAFHHAHUHG,
 * COOOOOM, AW FUCK ITS EVERYWHERE, COOOOOM, AWGAHUGHAHG. Aw fuck, aw fuck. oh jesus. 
 * ahhghhha, there you are, my slippery white goo to the world, my son, my son...Well, 
 * it's time to get breakfast...well a little coom first wouldn't hurt. */

use flameshot::ScreenArgs;
use raylib::{ffi::Image as FfiImage, prelude::*};
const SPOTLIGHT_TINT: Color = Color::new(0x00, 0x00, 0x00, 190);

fn main() {
    let raw =
        flameshot::execute(ScreenArgs::builder().raw().build()).expect("Failed to take screenshot");
    let dyna_img = raw
        .to_dynamic_image()
        .expect("Failed to convert dynamic image");
    let screenshot_image = dyna_img.to_rgba8();

    let (width, height) = screenshot_image.dimensions();
    let (mut rl, thread) = raylib::init()
        .title(env!("CARGO_BIN_NAME"))
        .fullscreen()
        .size(0, 0)
        .transparent()
        .vsync()
        .build();

    let screenshot_image = unsafe {
        Image::from_raw(FfiImage {
            // We can leak memory here because raylib will free the memory for us
            data: Box::new(screenshot_image.into_vec())
                .leak()
                .as_mut_ptr()
                .cast(),
            format: PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32,
            mipmaps: 1,
            width: width as i32,
            height: height as i32,
        })
    };
    let screenshot_texture = rl
        .load_texture_from_image(&thread, &screenshot_image)
        .expect("failed to load screenshot into a texture");
    #[cfg(feature = "dev")]
    let mut spotlight_shader = rl
        .load_shader(&thread, None, Some("shaders/spotlight.fs"))
        .expect("Failed to load spotlight shader");

    #[cfg(not(feature = "dev"))]
    let mut spotlight_shader =
        rl.load_shader_from_memory(&thread, None, Some(include_str!("../shaders/spotlight.fs")));
    let mut rl_camera = Camera2D::default();
    rl_camera.zoom = 1.0;
    // TODO: Get the current monitor position in virtual space and put the camera there
    let mut delta_scale = 0f64;
    let mut scale_pivot = rl.get_mouse_position();
    let mut velocity = Vector2::default();
    let mut spotlight_radius_multiplier = 1.0;
    let mut spotlight_radius_multiplier_delta = 0.0;

    #[cfg(feature = "dev")]
    let mut spotlight_tint_uniform_location;
    #[cfg(feature = "dev")]
    let mut cursor_position_uniform_location;
    #[cfg(feature = "dev")]
    let mut spotlight_radius_multiplier_uniform_location;
    #[cfg(not(feature = "dev"))]
    let spotlight_tint_uniform_location;
    #[cfg(not(feature = "dev"))]
    let cursor_position_uniform_location;
    #[cfg(not(feature = "dev"))]
    let spotlight_radius_multiplier_uniform_location;

    spotlight_tint_uniform_location = spotlight_shader.get_shader_location("spotlightTint");
    cursor_position_uniform_location = spotlight_shader.get_shader_location("cursorPosition");
    spotlight_radius_multiplier_uniform_location =
        spotlight_shader.get_shader_location("spotlightRadiusMultiplier");
    while !rl.window_should_close() {
        let key_pressed = rl.get_key_pressed();
        if key_pressed != Some(KeyboardKey::KEY_LEFT_CONTROL)
            && key_pressed != Some(KeyboardKey::KEY_LEFT_SHIFT)
            && key_pressed != Some(KeyboardKey::KEY_R)
            && key_pressed != None
        {
            break;
        }

        #[cfg(feature = "dev")]
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            spotlight_shader = rl
                .load_shader(&thread, None, Some("shaders/spotlight.fs"))
                .expect("Failed to load spotlight shader");
            spotlight_tint_uniform_location = spotlight_shader.get_shader_location("spotlightTint");
            cursor_position_uniform_location =
                spotlight_shader.get_shader_location("cursorPosition");
            spotlight_radius_multiplier_uniform_location =
                spotlight_shader.get_shader_location("spotlightRadiusMultiplier");
        }
        let enable_spotlight = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL);
        let scrolled_amount = rl.get_mouse_wheel_move_v().y * 1.3;
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT_CONTROL) {
            spotlight_radius_multiplier = 15.0;
            spotlight_radius_multiplier_delta = -15.0;
        }
        if scrolled_amount != 0.0 {
            match (
                enable_spotlight,
                rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT),
            ) {
                (_, false) => {
                    delta_scale += (scrolled_amount * 1.3) as f64;
                }
                (true, true) => {
                    spotlight_radius_multiplier_delta -= (scrolled_amount * -1.3) as f64;
                }
                _ => {}
            }
            scale_pivot = rl.get_mouse_position();
        }
        if delta_scale.abs() > 0.5 {
            let p0 = scale_pivot / rl_camera.zoom;
            rl_camera.zoom = (rl_camera.zoom as f64 + delta_scale * rl.get_frame_time() as f64)
                .clamp(1.0, 10.) as f32;
            let p1 = scale_pivot / rl_camera.zoom;
            rl_camera.target += p0 - p1;
            delta_scale -= delta_scale * rl.get_frame_time() as f64 * 4.0
        }
        spotlight_radius_multiplier = (spotlight_radius_multiplier as f64
            + spotlight_radius_multiplier_delta * rl.get_frame_time() as f64)
            .clamp(0.3, 10.) as f32;
        spotlight_radius_multiplier_delta -=
            spotlight_radius_multiplier_delta * rl.get_frame_time() as f64 * 4.0;
        const VELOCITY_THRESHOLD: f32 = 15.0;
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let delta = rl
                .get_screen_to_world2D(rl.get_mouse_position() - rl.get_mouse_delta(), rl_camera)
                - rl.get_screen_to_world2D(rl.get_mouse_position(), rl_camera);
            rl_camera.target += delta;
            velocity = delta * rl.get_fps().as_f32();
        } else if velocity.length_sqr() > VELOCITY_THRESHOLD * VELOCITY_THRESHOLD {
            rl_camera.target += velocity * rl.get_frame_time();
            velocity -= velocity * rl.get_frame_time() * 6.0;
        }

        let mut d = rl.begin_drawing(&thread);
        let mut mode2d = d.begin_mode2D(rl_camera);
        if enable_spotlight {
            mode2d.clear_background(SPOTLIGHT_TINT);
            let mouse_position = mode2d.get_mouse_position();
            spotlight_shader.set_shader_value(
                spotlight_tint_uniform_location,
                SPOTLIGHT_TINT.color_normalize(),
            );
            let screen_height = mode2d.get_screen_height().as_f32();
            spotlight_shader.set_shader_value(
                cursor_position_uniform_location,
                Vector2::new(mouse_position.x, screen_height - mouse_position.y),
            );
            spotlight_shader.set_shader_value(
                spotlight_radius_multiplier_uniform_location,
                spotlight_radius_multiplier,
            );

            let mut shader_mode = mode2d.begin_shader_mode(&spotlight_shader);
            shader_mode.draw_texture(&screenshot_texture, 0, 0, Color::WHITE);
        } else {
            mode2d.clear_background(Color::get_color(0));
            mode2d.draw_texture(&screenshot_texture, 0, 0, Color::WHITE);
        }
    }
}
