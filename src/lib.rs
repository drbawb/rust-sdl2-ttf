#[crate_id="github.com/drbawb/rust-sdl2_ttf#sdl2_ttf:0.1"];
#[crate_type="lib"];
#[feature(globs)];

extern crate sdl2;

use std::num::FromPrimitive;
use std::libc::{c_int, c_long};
use std::str;

use sdl2::sdl;
use sdl2::pixels::{Color,RGB,RGBA};
use sdl2::surface::Surface;

#[allow(non_camel_case_types)]
mod ffi;

#[cfg(test)]
mod tests;

pub enum FontStyle {
    NormalStyle = ffi::TTF_STYLE_NORMAL as int,
    BoldStyle = ffi::TTF_STYLE_BOLD as int,
    ItalicStyle = ffi::TTF_STYLE_ITALIC as int,
    UnderlineStyle = ffi::TTF_STYLE_UNDERLINE as int,
    StrikethroughStyle = ffi::TTF_STYLE_STRIKETHROUGH as int
}

#[deriving(FromPrimitive)]
pub enum FontHinting {
    NormalHinting = ffi::TTF_HINTING_NORMAL as int,
    LightHinting = ffi::TTF_HINTING_LIGHT as int,
    MonoHinting = ffi::TTF_HINTING_MONO as int,
    NoneHinting = ffi::TTF_HINTING_NONE as int
}

pub struct GlyphMetrics {
    minx: int,
    maxx: int,
    miny: int,
    maxy: int,
    advance: int
}

pub struct Font {
    priv raw: *ffi::TTF_Font
}

impl Drop for Font {

    fn drop(&mut self) {
        unsafe {
            ffi::TTF_CloseFont(self.raw);
        }
    }
}

impl Font {

    pub fn get_style(&self) -> ~[FontStyle] {
        let bitflags = unsafe { ffi::TTF_GetFontStyle(self.raw) };

        let flags = [
            NormalStyle,
            BoldStyle,
            ItalicStyle,
            UnderlineStyle,
            StrikethroughStyle
        ];

        flags.iter().filter_map(|&flag| {
            if bitflags & (flag as ffi::TTF_StyleFlag) != 0 {
                Some(flag)
            }
            else {
                None
            }
        }).collect()
    }

    pub fn set_style(&mut self, flags: &[FontStyle]) {
        let bitflags = flags.iter().fold(0, |bitflags, &flag| {
            bitflags | flag as ffi::TTF_StyleFlag
        });

        unsafe {
            ffi::TTF_SetFontStyle(self.raw, bitflags);
        }
    }

    pub fn get_outline(&self) -> int {
        unsafe {
            ffi::TTF_GetFontOutline(self.raw) as int
        }
    }

    pub fn set_outline(&mut self, outline: int) {
        unsafe {
            ffi::TTF_SetFontOutline(self.raw, outline as c_int);
        }
    }

    pub fn get_hinting(&self) -> FontHinting {
        unsafe {
            FromPrimitive::from_int(ffi::TTF_GetFontHinting(self.raw) as int).unwrap()
        }
    }

    pub fn set_hinting(&mut self, hinting: FontHinting) {
        unsafe {
            ffi::TTF_SetFontHinting(self.raw, hinting as ffi::TTF_Hinting);
        }
    }

    pub fn get_kerning(&self) -> int {
        unsafe {
            ffi::TTF_GetFontKerning(self.raw) as int
        }
    }

    pub fn set_kerning(&mut self, kerning: int) {
        unsafe {
            ffi::TTF_SetFontKerning(self.raw, kerning as c_int);
        }
    }

    pub fn height(&self) -> int {
        unsafe {
            ffi::TTF_FontHeight(self.raw) as int
        }
    }

    pub fn ascent(&self) -> int {
        unsafe {
            ffi::TTF_FontAscent(self.raw) as int
        }
    }

    pub fn descent(&self) -> int {
        unsafe {
            ffi::TTF_FontDescent(self.raw) as int
        }
    }

    pub fn line_skip(&self) -> int {
        unsafe {
            ffi::TTF_FontLineSkip(self.raw) as int
        }
    }

    pub fn faces(&self) -> int {
        unsafe {
            ffi::TTF_FontFaces(self.raw) as int
        }
    }

    pub fn face_is_fixed_width(&self) -> bool {
        unsafe {
            ffi::TTF_FontFaceIsFixedWidth(self.raw) > 0
        }
    }

    pub fn face_family_name(&self) -> Option<~str> {
        unsafe {
            let ptr = ffi::TTF_FontFaceFamilyName(self.raw);

            if ptr.is_null() {
                None
            } else {
                Some(str::raw::from_c_str(ptr))
            }
        }
    }

    pub fn glyph_is_provided(&self, glyph: char) -> Option<int> {
        let ch = match char_to_utf16(glyph) {
            Some(ch) => ch,
            None => return None
        };

        unsafe {
            match ffi::TTF_GlyphIsProvided(self.raw, ch) {
                0 => None,
                ch => Some(ch as int)
            }
        }
    }

    pub fn glyph_metrics(&self, glyph: char) -> Result<~GlyphMetrics, ~str> {
        let ch = match char_to_utf16(glyph) {
            Some(ch) => ch,
            None => return Err(~"Glyph is not a UTF-16 character")
        };

        let mut minx: c_int = 0;
        let mut maxx: c_int = 0;
        let mut miny: c_int = 0;
        let mut maxy: c_int = 0;
        let mut advance: c_int = 0;

        unsafe {
            match ffi::TTF_GlyphMetrics(self.raw, ch, &mut minx, &mut maxx,
                    &mut miny, &mut maxy, &mut advance) {
                0 => Ok(~GlyphMetrics {minx: minx as int, maxx: maxx as int,
                    miny: miny as int, maxy: maxy as int,
                    advance: advance as int}),
                _ => Err(sdl::get_error())
            }
        }
    }

    pub fn text_size(&self, text: &str) -> Result<(int, int), ~str> {
        let mut w: c_int = 0;
        let mut h: c_int = 0;

        text.with_c_str(|c_text| {
            unsafe {
                match ffi::TTF_SizeUTF8(self.raw, c_text, &mut w, &mut h) {
                    0 => Ok((w as int, h as int)),
                    _ => Err(sdl::get_error())
                }
            }
        })
    }
}

fn char_to_utf16(glyph: char) -> Option<u16> {
    // TODO bounds checking
    if !str::is_utf16([glyph as u16]) {
        return None;
    }

    Some(str::from_char(glyph).to_utf16()[0])
}

pub fn init() -> bool {
    unsafe {
        ffi::TTF_Init() == 0
    }
}

pub fn was_init() -> bool {
    unsafe {
        ffi::TTF_WasInit() == 1
    }
}

pub fn quit() {
    unsafe {
        ffi::TTF_Quit();
    }
}

pub fn open_font(file: &str, ptsize: int) -> Result<~Font, ~str> {
    file.with_c_str(|c_str| {
        unsafe {
            let ptr = ffi::TTF_OpenFont(c_str, ptsize as c_int);
            if ptr.is_null() {
                Err(sdl::get_error())
            } else {
                Ok(~Font {raw: ptr})
            }
        }
    })
}

pub fn open_font_index(file: &str, ptsize: int, index: int)
        -> Result<~Font, ~str> {
    file.with_c_str(|c_str| {
        let ptr = unsafe {
            ffi::TTF_OpenFontIndex(c_str, ptsize as c_int, index as c_long)
        };

        if ptr.is_null() {
            Err(sdl::get_error())
        } else {
            Ok(~Font {raw: ptr})
        }
    })
}

fn native_color(color: Color) -> sdl2::pixels::ll::SDL_Color {
	match color {
		RGB(r,g,b) 	=> sdl2::pixels::ll::SDL_Color { r: r, g: g, b: b, a: 255 },
		RGBA(r,g,b,a) 	=> sdl2::pixels::ll::SDL_Color {r: r, g: g, b: b, a: a },
	}
}

pub fn render_solid(font: &Font, text: &str, fg: Color)
        -> Result<~Surface, ~str> {
    text.with_c_str(|c_text| {
        let ptr = unsafe {
            ffi::TTF_RenderUTF8_Solid(font.raw, c_text, native_color(fg))
        };

        if ptr.is_null() {
            Err(sdl::get_error())
        } else {
            Ok(~Surface {raw: ptr, owned: true})
        }
    })
}

pub fn render_shaded(font: &Font, text: &str, fg: Color, bg: Color)
        -> Result<~Surface, ~str> {
    text.with_c_str(|c_text| {
        let ptr = unsafe {
            ffi::TTF_RenderUTF8_Shaded(font.raw, c_text, native_color(fg),
                native_color(bg))
        };

        if ptr.is_null() {
            Err(sdl::get_error())
        } else {
            Ok(~Surface {raw: ptr, owned: true})
        }
    })
}

pub fn render_blended(font: &Font, text: &str, fg: Color)
        -> Result<~Surface, ~str> {
    text.with_c_str(|c_text| {
        let ptr = unsafe {
            ffi::TTF_RenderUTF8_Blended(font.raw, c_text, native_color(fg))
        };

        if ptr.is_null() {
            Err(sdl::get_error())
        } else {
            Ok(~Surface {raw: ptr, owned: true})
        }
    })
}

