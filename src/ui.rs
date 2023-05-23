
use std::collections::HashSet;

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
        DrawParam, Mesh, MeshBuilder, MeshData, FillOptions, Canvas, FillRule, StrokeOptions, Drawable
    }, 
    glam::Vec2, 
    conf, timer::{TimeContext, self}};
use ggez::event;

#[derive(Clone)]
pub struct Stroke{
    color:Color,
    width:f32
}
impl Stroke{
    pub fn new()->Self{
        Stroke { 
            color: Color { 
                r: 0.0, 
                g: 0.0, 
                b: 0.0, 
                a: 0.0 
            }, 
            width: 0.0 
        }
    }
    pub fn color(&mut self, c:Color) ->&mut Self{
        self.color = c;
        return self;
    }
    pub fn width(&mut self, w:f32) ->&mut Self{
        self.width = w;
        return self;
    }

}
#[derive(Clone)]
pub struct Style{
    fg:Color,
    bg:Color,
    stroke:Stroke,
    radius:f32,
    margin:f32,
    padding:f32

}

impl Style{
    pub fn new() -> Self{
        Style{
            fg:Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            bg:Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            stroke:Stroke::new(),
            radius:0.0,
            margin:0.0,
            padding:0.0
        }
    }
    pub fn fg(&mut self, color:Color) -> &mut Self{
        self.fg = color;
        return self;
    }

    pub fn bg(&mut self, color:Color) -> &mut Self{
        self.bg = color;
        return self;
    }

    pub fn stroke(&mut self, stroke:Stroke) -> &mut Self{
        self.stroke = stroke;
        return self;
    } 
    pub fn radius(&mut self, rad:f32) -> &mut Self{
        self.radius = rad;
        return self;
    }
    pub fn margin(&mut self, margin:f32) -> &mut Self{
        self.margin = margin;
        return self;
    }
    pub fn padding(&mut self, padding:f32) -> &mut Self{
        self.padding = padding;
        return self;
    }
}


#[derive(Clone)]
pub struct Panel{
    rect:Rect,
    style:Style,
    fill:Mesh,
    stroke:Mesh,

}


impl Panel {
    pub fn new(
        ctx:&mut Context, 
        rect:&mut Rect, 
        style:Style
    )->Self{
        rect.w -= style.padding*2.0;
        rect.h -= style.padding*2.0;
        rect.x += style.padding;
        rect.y += style.padding;

        let mut fill_mb = MeshBuilder::new();
        let mut stroke_mb = MeshBuilder::new();

        let fill_opt = DrawMode::Fill( FillOptions::default() );
        let stroke_opt = DrawMode::Stroke( 
            StrokeOptions::default()
                .with_line_width(style.stroke.width) );

        let fill_rect = fill_mb.rounded_rectangle (
            fill_opt, 
            *rect, 
            style.radius, 
            style.bg
        ).unwrap();
        
        let stroke_rect = stroke_mb.rounded_rectangle(
            stroke_opt, 
            *rect, 
            style.radius, 
            style.stroke.color
        ).unwrap();
        
        let fill = Mesh::from_data(&ctx.gfx, fill_rect.build());
        let stroke = Mesh::from_data(&ctx.gfx, stroke_rect.build());


        return Panel { rect:*rect, style, fill, stroke };
    }

    /*pub fn draw(&self, canvas:&mut Canvas, draw_param:DrawParam){
        //canvas.set_sampler(graphics::Sampler::linear_clamp());
        canvas.draw(&self.fill, draw_param);
        canvas.draw(&self.stroke, draw_param);
        //println!("Panel.draw()");
    }*/
}

impl Drawable for Panel{
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
        let mut param_binding = DrawParam::new();
        param_binding = param.into();
        canvas.draw(&self.fill, param_binding);
        canvas.draw(&self.stroke, param_binding); 
    }
    fn dimensions(&self, gfx: &impl ggez::context::Has<graphics::GraphicsContext>) -> Option<Rect> {
        return Some(self.rect);
    }
}

#[derive(Clone)]
pub struct Container{
    children:Vec<Container>,
    panel:Panel
}

impl Container {
    pub fn new(ctx:&mut Context, mut rect:Rect) -> Self{
        let style = Style::new();
        //let mut rect = rect;
        return Container{
            children:Vec::<Container>::new(),
            panel:Panel::new(ctx,&mut rect, style) 
        };
    }
    /*pub fn draw(&self, canvas:&mut Canvas, draw_param:DrawParam){
        //println!("Container.draw()");
        //self.panel.draw(canvas, draw_param);
        self.get_panel().fill.draw(canvas, draw_param);
        self.get_panel().stroke.draw(canvas, draw_param);
    }*/
    pub fn add_child(&mut self, c:Container) -> &Self{
        self.children.push(c);
        return self;
    }
    pub fn style(&mut self, s: Style) -> &Self{
        self.panel.style = s;
        return self;
    }
    pub fn rect(&mut self, r:Rect) -> &Self{
        self.panel.rect = r;
        return self;
    }

    pub fn full_screen(ctx:&mut Context) -> Container{
        let bind = Container::new(ctx, 
            Rect::new(0.0,0.0,
                ctx.gfx.window().inner_size().width as f32,
                ctx.gfx.window().inner_size().height as f32
        ));
        println!("Container::full_screen()");
        return bind.clone();
    }
    pub fn get_panel(&self) -> &Panel{
        return &self.panel;
    }
}

impl Drawable for Container{
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
        canvas.draw(&self.panel, param);
    }
    fn dimensions(&self, gfx: &impl ggez::context::Has<graphics::GraphicsContext>) -> Option<Rect> {
        return Some(self.panel.rect);
    }
}


