use ggez::graphics::{Image, Rect};

//returns a vec of uv rects
//image agnostic
//will work as intended reguardless of image path
//provided the path shares the same qualities ad the
//animation defined though the arguements.
pub fn sprite_animation(
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
    for f in 1..length{
        let frame = f as f32 * (first_last.0+1.0);
        println!("frame:{}",frame);
        out.push(Rect{x:x*frame, y:y, w:w, h:h});
    }
    return out;
}
// a modified version of this function could be used to manage tilesets.

pub struct AnimatedSprite{
    spite_sheet:Image,
    rows:i32,
    cols:i32,
}

impl AnimatedSprite {
    pub fn new(sprite_sheet:Image, rows:i32, cols:i32) -> Self{
        return AnimatedSprite{
            spite_sheet:sprite_sheet,
            rows:rows,
            cols:cols
        };
    }
    pub fn get_slice(&self, col:i32, first:i32, last:i32) -> Vec<Rect>{
        return sprite_animation(
            self.rows as f32, 
            self.cols as f32, 
            (first as f32, last as f32), 
            col as f32);
    }
}
