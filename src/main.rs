//This exampe should draw an image on the screen. SHOULD.
//so far i have been unable to achieve this behavior on
//my machine. 
//
//::SYS INFO::
//uname -a
//  Linux knoll 5.19.0-41-generic 
//  #42~22.04.1-Ubuntu 
//  x86_64 GNU/Linux
//

#![allow(dead_code, unused_variables, unused_imports)]
use std::{path::{PathBuf, self}, fs, io::Read, env, fmt::format};
use core::slice::Iter;
use std::fmt::Write;
use ggez::{
    Context, 
    GameResult, 
    GameError, 
    event::EventHandler,
    graphics::{
        Image, 
        self, 
        Color, 
        Mesh, 
        MeshBuilder,
        MeshData, DrawMode, Rect, DrawParam
    }, 
    glam::Vec2, 
    conf};
use ggez::event;
use keyframe::{ease, functions::*, keyframes, AnimationSequence, EasingFunction};
use keyframe_derive::CanTween;


fn main() {
    // set resouce dir (target/debug/resources) or (target/release/resources)
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources/");
        path
    }else{
        PathBuf::from("target/release/resources/")
    };
    //default config
    let conf = conf::Conf::new();
    //context builder
    let cb = ggez::ContextBuilder::new("magic-mania", "strange-knoll")
        .add_resource_path(resource_dir)
        .default_conf(conf);

    let (mut ctx, events_loop) = cb.build().ok().unwrap();

    let state = Dungeon::new(&mut ctx);

    event::run(ctx, events_loop, state)
}


#[derive(CanTween, Clone, Copy)]
/// necessary because we can't implement CanTween for graphics::Rect directly, as it's a foreign type
struct TweenableRect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl TweenableRect {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        TweenableRect { x, y, w, h }
    }
}

impl From<TweenableRect> for graphics::Rect {
    fn from(t_rect: TweenableRect) -> Self {
        graphics::Rect {
            x: t_rect.x,
            y: t_rect.y,
            w: t_rect.w,
            h: t_rect.h,
        }
    }
}

/// A fancy easing function, tweening something into one of `frames` many discrete states.
/// The `pre_easing` is applied first, thereby making other `EasingFunction`s usable in the realm of frame-by-frame animation
struct AnimationFloor {
    pre_easing: Box<dyn EasingFunction + Send + Sync>,
    frames: i32,
}
impl EasingFunction for AnimationFloor {
    #[inline]
    fn y(&self, x: f64) -> f64 {
        (self.pre_easing.y(x) * (self.frames) as f64).floor() / (self.frames - 1) as f64
    }
}

fn player_sequence(
    ease_enum: &EasingEnum,
    anim_type: &AnimationType,
    size:(usize, usize),        //width and hight of the sprite frame
    col_row: (usize, usize),    //rows (frames) of the animation and 
                                //cols(animations) in the spritesheet
    first_last:(usize, usize),  //the first and last index of frames taken from spritesheet
    index: usize,               //which animation (col) in sprite sheet we want to play
    duration:f32
) -> AnimationSequence<TweenableRect> {
    // create the two Rects that will serve as `from` and `to` for the DrawParam::src of the animation
    // the start for all animations is at the leftmost frame, starting at 0.0
    let src_x_start: f32 = 0.0;
    // the final parameter depends upon how many frames there are in an animation
    let src_x_end = col_row.0;
    // the src.y parameter depends on the row in which the animation is placed inside the sprite sheet
    let src_y = col_row.1;
    // the height and width of the source rect are the proportions of a frame relative towards the whole sprite sheet
    let w = 1.0 / col_row.0 as f32;
    let h = 1.0 / col_row.1 as f32;
    let src_rect_start = TweenableRect::new(src_x_start*first_last.0, src_y, w, h);
    let src_end_rect = TweenableRect::new(src_x_end*first_last.1, src_y, w, h);

    let frames = first_last.1-first_last.0;

    if let EasingEnum::EaseInOut3Point = ease_enum {
        // first calculate the middle state of this sequence
        // luckily we can use keyframe to help us with that
        let mid = ease(
            AnimationFloor {
                pre_easing: Box::new(Linear),
                frames,
            },
            src_rect_start,
            src_end_rect,
            0.33,
        );
        let mid_frames = (frames as f32 * 0.33).floor() as i32;
        // we need to adapt the frame count for each keyframe
        // only the frames that are to be played until the next keyframe count
        keyframes![
            (
                src_rect_start,
                0.0,
                AnimationFloor {
                    pre_easing: Box::new(EaseInOut),
                    frames: mid_frames + 1
                }
            ),
            (
                mid,
                0.66 * duration,
                AnimationFloor {
                    pre_easing: Box::new(EaseInOut),
                    frames: frames - mid_frames
                }
            ),
            (src_end_rect, duration)
        ]
    } else {
        // the simpler case: choose some easing function as the pre-easing of an AnimationFloor
        // which operates on all frames, from the first to the last
        let easing = AnimationFloor {
            pre_easing: easing_function(ease_enum),
            frames,
        };
        keyframes![
            (src_rect_start, 0.0, easing),
            (src_end_rect, duration) // we don't need to specify a second easing function,
                                     // since this sequence won't be reversed, leading to
                                     // it never being used anyway
        ]
    }
}

// load file from bytes (credit: Bowarc)
fn load_file(p: &str) -> Option<Vec<u8>>{
    //build path
    let path = PathBuf::from(format!("{}", p));
    //chech path
    if !path.exists(){
        println!("Path doesn't exist: {path:?}");
        return None;
    }
    //debug print our path
    println!("Found image path: {}", path.display());
    
    //read file from path and write data to bites
    let mut file = fs::File::open(path).ok()?;
    let mut bytes:Vec<u8> = Vec::new();
    //writes Vec<u8> to bytes
    //_bytes_read returns num of bytes read
    let _bytes_read = file.read_to_end(&mut bytes);
    
    // <DEBUG>
    println!("DUMP IMG BYTES ...");
    println!("{}", _bytes_read.unwrap());
    //<BoilerPlate>
    //constructs printable string of hex values
    let mut s = String::new();
    for &b in bytes.iter(){
        write!(&mut s, "{:X} ", b).expect("Unable to write");
    }
    //</BoilerPlate>

    // dump bytes to console
    println!("{}", s);
    // </DEBUG>
    
    Some(bytes) // return some bytes :p
}

struct Dungeon{
    sprite_sheet:Image, 
    sprite:Image,
    player_animation:AnimationSequence<TweenableRect>,
}

impl Dungeon{
    fn new(ctx: &mut Context) -> Dungeon{
        //build path
        let path = format!("{}/{}", 
            ctx.fs.resources_dir().display(),
            "Icon.1_04.png"
        );
        //debug path
        println!("loaded path: {}", path);
        //load bytes from file at path
        let bytes = load_file(path.as_str());
        //creates an image from loaded bytes
        let img = Image::from_bytes(&ctx.gfx, &bytes.unwrap()).unwrap();

        let sheet_bytes = load_file(
            format!("{}/{}",
                ctx.fs.resources_dir().display(),
                "characters_7.png"
            ).as_str()
        );
        let sheet = Image::from_bytes(
            &ctx.gfx, &sheet_bytes.unwrap())
            .unwrap();

        //returns dungeon with our image as sprite
        return Dungeon{
            sprite_sheet:sheet,
            sprite:img,
            player_animation:player_sequence(&EasingEnum::Linear, &AnimationType::Idle, 1.0)
        }
    }
}

impl EventHandler for Dungeon{
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // base canvas
        let mut canvas = graphics::Canvas::from_frame(&ctx.gfx, Color::BLACK);

        //transform
        let dest = Vec2::new(256.0,256.0);
        //draw our sprite
        canvas.draw(
            &self.sprite.clone(), 
            graphics::DrawParam::new()
                .scale(Vec2::new(1.0, 1.0))
                .dest(dest)
                .z(0)
        );

        // draw the player
        let current_frame_src: graphics::Rect = self.player_animation.now_strict().unwrap().into();
        let scale = 3.0;
        canvas.draw(
            &self.sprite_sheet,
            graphics::DrawParam::new()
                .src(current_frame_src)
                .scale([scale, scale])
                .dest([470.0, 460.0])
                .offset([0.5, 1.0]),
        );


        canvas.finish(&mut ctx.gfx)?;

        Ok(())
    }
}
