use std::{rc::Rc};
use sdl2::{
    self,
    Sdl, EventPump,
    render::{WindowCanvas, TextureAccess},
    pixels::PixelFormatEnum,
    event::Event,
    keyboard::{Keycode, Scancode},
    mouse::{MouseState, RelativeMouseState},
};

rental! { mod rent {
    use sdl2::{
        video::{WindowContext},
        render::{TextureCreator, Texture},
    };

    #[rental_mut]
    pub struct RentTexture {
        creator: Box<TextureCreator<WindowContext>>,
        texture: Texture<'creator>,
    }
}}
use rent::{RentTexture};

pub struct Window {
    context: Rc<Sdl>,
    size: (usize, usize),
    canvas: WindowCanvas,
    texture: Option<RentTexture>,
}

impl Window {
    pub fn new(context: Rc<Sdl>, size: (usize, usize), title: &str) -> base::Result<Self> {
        let video = context.video()?;
     
        let window = video.window(title, size.0 as u32, size.1 as u32)
        .position_centered()/*.resizable()*/.build()
        .map_err(|e| e.to_string())?;
     
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();
        let texture = Some(RentTexture::try_new_or_drop(
            Box::new(texture_creator),
            |tc| {
                tc.create_texture(
                    PixelFormatEnum::RGB24,
                    TextureAccess::Streaming,
                    size.0 as u32,
                    size.1 as u32,
                ).map_err(|e| e.to_string())
            }
        )?);

        Ok(Self {
            context,
            size,
            canvas,
            texture,
        })
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn draw(&mut self/*, img: &Image*/) -> base::Result<()> {
        //let mut texture = self.texture.take().unwrap();

        /*
        if let Some(ll) = self.state.screenshot {
            println!("saving screenshot ...");
            match save_screenshot(img, ll) {
                Ok(f) => println!("... saved to '{}'", f),
                Err(e) => eprintln!("error saving screenshot: {}", e),
            }
            self.state.screenshot = None;
        }
        */

        /*
        let res = img.read()
        .and_then(|data| {
            texture.rent_mut(|texture| {
                texture.update(None, &data, 3*img.dims().0)
            }).map_err(|e| base::Error::from(e.to_string()))
        })
        .and_then(|()| {
            //self.canvas.clear();
            texture.rent(|texture| {
                self.canvas.copy(texture, None, None)
                .map_err(|e| base::Error::from(e))
            })
            .map(|()| self.canvas.present())
        });
        */

        //assert!(self.texture.replace(texture).is_none());

        //res
        Ok(())
    }
} 
