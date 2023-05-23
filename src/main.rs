

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
        DrawParam, Mesh, MeshBuilder, MeshData
    }, 
    glam::Vec2, 
    conf, timer::{TimeContext, self}};
use ggez::event;

mod image_loader;
use image_loader::load_file;
mod animated_sprite;
use animated_sprite::{AnimatedSprite, SpriteSheet};
mod ui;
use ui::*;


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
    tick:usize,
    panel:Panel,
    ui:Panel
}

impl Dungeon{
    fn new(ctx: &mut Context) -> Dungeon{

        // building character sprite
        let character_string = "characters_7.png".to_string();
        
        let character_sprite = SpriteSheet::new(ctx,
            character_string,23,4);

        let character_anim = AnimatedSprite::new(
            character_sprite.clone(), 0, 14, 18);

        // build style for fullscreen ui container
        let mut style_binding = Style::new();
        let style = style_binding
            .fg(Color::WHITE)
            .bg(Color::RED)
            .radius(8.0)
            .padding(16.0)
            .stroke(
                Stroke::new()
                    .color(Color::WHITE)
                    .width(2.0)
                    .clone()
            );
        
        //build fullscreen container
        let mut fullscreen_rect_binding = Container::full_screen(ctx);
        let fullscreen_rect = fullscreen_rect_binding
            .style(style.clone())
            .get_panel();
       
        //stroke for panel
        let mut panel_stroke_binding = Stroke::new();   // theres a lot of these bindings
                                                        // im not really sure what this is doing
                                                        // just following the compilers wisdom   
        let panel_stroke = panel_stroke_binding
            .color(Color::RED)
            .width(8.0);
        //style for panel
        let mut panel_style_binding = Style::new();
        let panel_style = panel_style_binding
            .bg(Color::BLUE)
            .stroke(panel_stroke.clone())
            .radius(24.0);
        //panel
        let panel = Panel::new(ctx, 
            &mut Rect::new(100.0,100.0, 200.0, 200.0), panel_style.clone());
        //returns dungeon with our image as sprite
        return Dungeon{
            sprite_sheet:character_sprite,
            player_animation:character_anim,
            time:TimeContext::new(),
            tick:0,
            panel:panel,
            ui:fullscreen_rect.clone()
        }
    }
}




impl EventHandler for Dungeon{
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // animate player sprite
        self.player_animation.update(ctx, 8).unwrap();
        
        Ok(())
    }
    

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // base canvas
        let mut canvas = graphics::Canvas::from_frame(&ctx.gfx, Color::BLACK);

        canvas.set_sampler(graphics::Sampler::nearest_clamp()); // because pixel art
        
        //player has not been implimented with Drawable
        //hence the reaso we call the players draw
        self.player_animation.draw(
            &mut canvas,
            DrawParam::new()
                .offset([0.5,0.5])
                .scale([4.0,4.0])
                .dest([400.0,400.0])
        ); 
        
        //content and panel both have Drawable implimented
        //so we call them with canvas.draw
        canvas.draw(&self.ui, DrawParam::new());
        canvas.draw(&self.panel, DrawParam::new());
        canvas.finish(&mut ctx.gfx)?;

        Ok(())
    }
}
