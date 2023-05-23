

use ggez::{
    Context,
    graphics::{
        Image, 
        Rect, 
        Color, 
        Sampler, 
        Canvas, self,
        Drawable, DrawParam
    }, 
    event::EventHandler, 
    GameError, timer, glam::Vec2, mint::Vector2
};

use crate::image_loader;

//returns a vec of uv rects
//image agnostic
//will work as intended reguardless of image path
//provided the path shares the same qualities ad the
//animation defined though the arguements.
fn sprite_animation(
    rows:f32, cols:f32, 
    first_last:(f32, f32),
    col_indx:f32,
) -> Vec<Rect> {
    println!("spite_anim()");
    let w:f32 = 1.0/rows;
    let h:f32 = 1.0/cols;
    let x:f32 = w;
    let y:f32 = col_indx*h;
    println!("x:{}, y:{}, w:{}, h{}", x,y,w,h);
    let length = (first_last.1 - first_last.0).floor() as i32;
    println!("length:{}", length);
    let mut out = Vec::<Rect>::new();
    let first = first_last.0.floor() as i32;
    let last = first_last.1.floor() as i32;
    for f in first..last{
        let frame = f as f32;
        println!("frame:{}",frame);
        out.push(Rect{x:x*frame, y:y, w:w, h:h});
    }
    return out;
}
// a modified version of this function could be used to manage tilesets.
#[derive(Clone)]
struct transform{
    dest: Vector2<f32>,
    offset: Vector2<f32>,
    scale: Vector2<f32>,
    z: i32,
}


#[derive(Clone)]
pub struct SpriteSheet{
    pub image:Image,
    pub rows:i32,
    pub cols:i32
}

impl SpriteSheet{
    pub fn new(ctx: &mut Context, path:String, rows:i32, cols:i32) -> Self{
        let path = format!("{}/{}",
            ctx.fs.resources_dir().display(), path);
        let bytes = image_loader::load_file(path.as_str()).unwrap();
        let image = Image::from_bytes(&mut ctx.gfx, &bytes).unwrap();
        return SpriteSheet{image,rows, cols};
    }
}

#[derive(Clone)]
pub struct AnimatedSprite{
    anim:Vec<Rect>,
    sprite_sheet:SpriteSheet,
    column:i32,
    first_frame:i32,
    last_frame:i32,
    tick:i32,

}

impl AnimatedSprite {
    pub fn new(sprite_sheet:SpriteSheet, 
        column:i32, first:i32, last:i32) -> Self{
        return AnimatedSprite { 
            anim:sprite_animation(
                sprite_sheet.rows as f32, 
                sprite_sheet.cols as f32, 
                (first as f32, last as f32), 
                column as f32
            ),
            sprite_sheet: sprite_sheet, 
            column: column, 
            first_frame: first, 
            last_frame: last,
            tick:0,

        };
    }
    pub fn sprite_sheet(&self) -> Image{
        return self.sprite_sheet.image.clone();
    }
    pub fn get_frames(&self) -> Vec<Rect>{
        return self.anim.clone();
    }

}

impl AnimatedSprite{
    pub fn update(&mut self, ctx: &mut ggez::Context, tick_rate:u32) -> Result<(), GameError> {
        while(timer::check_update_time(ctx, tick_rate)){
            //println!("Tick: {}", self.tick);
            self.tick+=1;
        } 
        Ok(())
    }

    pub fn draw(&mut self, 
        canvas: &mut Canvas, draw_param:DrawParam
    ) {
        //canvas.set_sampler(Sampler::nearest_clamp()); // because pixel art
        
        // draw the player
        let current_frame_src = self.anim.get(
            self.tick as usize % self.anim.len()
        ).unwrap();

        canvas.draw(
            &self.sprite_sheet.image,
            draw_param.src(current_frame_src.clone())

            )
    }
}
