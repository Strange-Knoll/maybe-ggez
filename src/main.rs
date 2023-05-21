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

#![allow(dead_code, /*unused_variables,*/ unused_imports)]
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
        DrawMode, 
        Rect, 
        DrawParam
    }, 
    glam::Vec2, 
    conf, timer::{TimeContext, self}};
use ggez::event;

mod image_loader;
use image_loader::*;
mod animated_sprite;
use animated_sprite::*;


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



struct Dungeon{
    sprite_sheet:Image, 
    sprite:Image,
    player_animation:Vec<Rect>,
    time:TimeContext,
    tick:usize
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
                "dva/tentai/missionary_face_fuck/full.png"
            ).as_str()
        );
        let sheet = Image::from_bytes(
            &ctx.gfx, &sheet_bytes.unwrap())
            .unwrap();

        //returns dungeon with our image as sprite
        return Dungeon{
            sprite_sheet:sheet,
            sprite:img,
            player_animation:
                sprite_animation(6.0, 1.0, (0.0,6.0), 0.0),
            time:TimeContext::new(),
            tick:0
        }
    }
}




impl EventHandler for Dungeon{
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let secs = ctx.time.delta().as_secs_f64();
        // advance the ball animation and reverse it once it reaches its end
        //self.ball_animation.advance_and_maybe_reverse(secs);
        // advance the player animation and wrap around back to the beginning once it reaches its end
        while(timer::check_update_time(ctx, 8)){
            println!("Tick: {}", self.tick);
            self.tick+=1;
        }
        
        Ok(())
    }
    

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // base canvas
        let mut canvas = graphics::Canvas::from_frame(&ctx.gfx, Color::BLACK);
        canvas.set_sampler(graphics::Sampler::nearest_clamp()); // because pixel art


        //draw our sprite
        canvas.draw(
            
            &self.sprite.clone(), 
            graphics::DrawParam::new()
                .scale([0.25, 0.25])
                .dest([20.0,20.0])
                .z(-1)
        );

        // draw the player
        let current_frame_src = self.player_animation.get(
            self.tick % self.player_animation.len()
        ).unwrap(); 
        let scale = 8.0;
        canvas.draw(
            &self.sprite_sheet,
            graphics::DrawParam::new()
                .src(current_frame_src.clone())
                .scale([scale, scale])
                .dest([16.0*scale, 16.0*scale])
                .offset([0.5, 0.5])
                .z(5),
        );


        canvas.finish(&mut ctx.gfx)?;

        Ok(())
    }
}
