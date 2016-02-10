use core::Core;
use core::syntax::Color;
use core::view::{ Line};
use core::syntax::Highlighter;

use cairo::{Context, RectangleInt};
use gtk::DrawingArea;
use gtk::signal::Inhibit;
use gtk::traits::*;

use pango::ffi::PANGO_SCALE;
use pangocairo::attribute::PangoAttribute;
use pangocairo::layout::PangoLayout;

use std::cell::RefCell;
use std::rc::Rc;
struct Font {
    font: String,
    size: Option<(usize, usize)>
}
impl Font {
    
    pub fn new(font:String) -> Font {
        Font {
            font: font,
            size: None
        }
    }
    
    pub fn set_font(&mut self, font: String) {
        self.font = font;
        self.size = None;
    }
    ///panic if error    
    fn draw_test_line(&self, context: &mut Context) -> (usize, usize) {
        const TEST_STRING:&'static str = "test";
        context.push_group();
        let size = {
            PangoLayout::new(context, &*self.font)
                .push(TEST_STRING,vec![PangoAttribute::new_bold()])
                .update()
                .get_size()
        };
        context.pop_group_to_source();
        assert!(size.0 >= 0);
        assert!(size.1 >= 0);  
        (size.0 as usize, size.1 as usize)
    }
    ///panic if error
    pub fn size(&mut self, context: &mut Context) -> (usize, usize) {
        self.size = self.size.or_else(|| {
            Some(self.draw_test_line(context))
        });
        self.size.unwrap()
    }
    ///panic if error
    pub fn height(&mut self, context: &mut Context) -> usize {
        self.size(context).1
    }
    ///panic if error
    pub fn width(&mut self, context: &mut Context) -> usize {
        self.size(context).0
    }
}
pub struct LineRenderer {
    core: Rc<Core>,
    highlighter:  Highlighter,
    fg_color: Color,
    bg_color: Color,
    height: usize,
    width: usize,
    ///interval between lines,
    interval: f64, 
    font: Font,
}

fn color_to_cairo(c: u8) -> f64 {
    c as f64 / 255.
}

impl LineRenderer {
    pub fn new_drawing_area(width:usize, height:usize, interval:f64,
              font:String, core: Rc<Core>, fg_color: Color, bg_color: Color) 
              -> DrawingArea {
        let line_renderer = Rc::new(RefCell::new(LineRenderer {
            height: height,
            width: width,
            fg_color: fg_color,
            bg_color: bg_color,

            interval: interval,
            font: Font::new(font),
            highlighter: core.create_highlighter().unwrap(),
            core: core,

        }));
        
        LineRenderer::init_callbacks(DrawingArea::new().unwrap(), line_renderer)
        
    }
    pub fn init_callbacks(drawing_area: DrawingArea,
            line_renderer: Rc<RefCell<LineRenderer>>) -> DrawingArea {
        // on draw
        {
        let line_renderer = line_renderer.clone();
        drawing_area.connect_draw( move |_, mut cr: Context| {
            line_renderer
                .borrow_mut()
                .on_draw(&mut cr);
            Inhibit(false)
        });
        }
        // on resize
        drawing_area.connect_size_allocate(move |_, rectangle: &RectangleInt| {
            line_renderer
                .borrow_mut()
                .on_size_alloc(rectangle);
        });
        drawing_area
    
    }
    fn render_line(&self,  cr: &mut Context, line: &Line) {
        let mut layout = PangoLayout::new(cr, &*self.font.font);
       
        for (style, text) in line.highlight(&self.highlighter) {
            let mut attributes = Vec::new();
            
            attributes.push(
                PangoAttribute::from_fg_color((
                    (style.foreground.r as u16) << 8,
                    (style.foreground.g as u16) << 8,
                    (style.foreground.b as u16) << 8
            )));
            attributes.push(
                PangoAttribute::from_bg_color((
                    (style.background.r as u16) << 8,
                    (style.background.g as u16) << 8,
                    (style.background.b as u16) << 8
            )));

            // replace copy by slicing
            layout.push(&*text, attributes);
        }
        layout.update().show();
    }
    
    fn on_size_alloc(&mut self, rectangle: &RectangleInt) {
        self.width = rectangle.width as usize;
        self.height = rectangle.height as usize;
    }
    fn on_draw(&mut self, cr: &mut Context){
        
        //cr.scale(height as f64, width as f64);
        let line_height = self.font.height(cr) as f64 / PANGO_SCALE as f64;
        let interval    = line_height * self.interval;
        let line_count  = self.height / interval as usize;

        
        cr.set_source_rgb (color_to_cairo(self.bg_color.r), 
                            color_to_cairo(self.bg_color.g), 
                            color_to_cairo(self.bg_color.b) );
        cr.rectangle( 0., 0., self.width as f64, self.height as f64);
        cr.fill();

        cr.set_source_rgb (color_to_cairo(self.fg_color.r), 
                            color_to_cairo(self.fg_color.g), 
                            color_to_cairo(self.fg_color.b) );
        for (line, y) in self.core.view.lines.iter()
                                             .zip((0..line_count)
                                                  .map(| i | i as f64 * interval )) {
            cr.move_to(0f64, y);
            self.render_line(cr,line);
        }
    }
}

