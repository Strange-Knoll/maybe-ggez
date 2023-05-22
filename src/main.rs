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
use image_loader::load_file;
mod animated_sprite;
use animated_sprite::{AnimatedSprite, SpriteSheet};


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
    sprite_sheet:SpriteSheet, 
    player_animation:AnimatedSprite,
    time:TimeContext,
    tick:usize
}

impl Dungeon{
    fn new(ctx: &mut Context) -> Dungeon{
        
        //creates an image from loaded bytes
        /*
        * still bugged in 0.9.0 rc 
        * let img = Image::from_path(ctx, path.as_str()).unwrap();
        */


        let dva_sprite = SpriteSheet::new(ctx,
            "dva/tentai/missionary_tits_fuck/full.png".to_string(),
            6,1);
            
        let dva_anim = AnimatedSprite::new(
            dva_sprite.clone(), 0, 0, 6)
            .transform(
                [0.0, 0.0].into(), 
                [0.0, 0.0].into(),
                [8.0, 8.0].into(), 
                0
            );

        //returns dungeon with our image as sprite
        return Dungeon{
            sprite_sheet:dva_sprite,
            player_animation:dva_anim,
            time:TimeContext::new(),
            tick:0
        }
    }
}




impl EventHandler for Dungeon{
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.player_animation.update(ctx, 8).unwrap();
        
        Ok(())
    }
    

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // base canvas
        let color = Color::new(0.0,0.0,0.0,0.0);
        let mut canvas = graphics::Canvas::from_frame(&ctx.gfx, Color::BLUE);
        canvas.set_sampler(graphics::Sampler::nearest_clamp()); // because pixel art
        self.player_animation
            /*.transform(
                [128.0,256.0].into(), 
                [0.5,0.5].into(), 
                [8.0,8.0].into(), 
                5 )*/
            .draw(ctx, &mut canvas); 


        canvas.finish(&mut ctx.gfx)?;

        Ok(())
    }
}
