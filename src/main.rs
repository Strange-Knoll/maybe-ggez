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

//#![allow(dead_code, unused_variables, unused_imports)]



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


// assumes a fixed frame rate
// return some iterator of frames
fn sprite_anim(
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
    //println!("DUMP IMG BYTES ...");
    //println!("{}", _bytes_read.unwrap());
    //<BoilerPlate>
    //constructs printable string of hex values
    let mut s = String::new();
    for &b in bytes.iter(){
        write!(&mut s, "{:X} ", b).expect("Unable to write");
    }
    //</BoilerPlate>

    // dump bytes to console
    //println!("{}", s);
    // </DEBUG>
    
    Some(bytes) // return some bytes :p
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
            player_animation:
                sprite_anim(23.0, 4.0, (0.0,5.0), 1.0),
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

        //transform
        let dest = Vec2::new(256.0,256.0);
        //draw our sprite
        canvas.draw(
            
            &self.sprite.clone(), 
            graphics::DrawParam::new()
                .scale(Vec2::new(0.5, 0.5))
                .dest([20.0,20.0])
                .z(0)
        );

        // draw the player
        let current_frame_src = self.player_animation.get(
            self.tick % self.player_animation.len()
        ).unwrap(); 
        let scale = 3.0;
        canvas.draw(
            &self.sprite_sheet,
            graphics::DrawParam::new()
                .src(current_frame_src.clone())
                .scale([scale, scale])
                .dest([470.0, 460.0])
                .offset([0.5, 1.0]),
        );


        canvas.finish(&mut ctx.gfx)?;

        Ok(())
    }
}
