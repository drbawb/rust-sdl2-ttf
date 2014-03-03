use std::libc::{c_int, c_char, c_void, c_long};
use sdl2::pixels::ll::SDL_Color;
use sdl2::surface::ll::SDL_Surface;

pub type TTF_Font = c_void;

pub type TTF_StyleFlag = c_int;
pub static TTF_STYLE_NORMAL: TTF_StyleFlag = 0x00;
pub static TTF_STYLE_BOLD: TTF_StyleFlag = 0x01;
pub static TTF_STYLE_ITALIC: TTF_StyleFlag = 0x02;
pub static TTF_STYLE_UNDERLINE: c_int = 0x04;
pub static TTF_STYLE_STRIKETHROUGH: c_int = 0x08;

pub type TTF_Hinting = c_int;
pub static TTF_HINTING_NORMAL: TTF_Hinting = 0;
pub static TTF_HINTING_LIGHT: TTF_Hinting = 1;
pub static TTF_HINTING_MONO: TTF_Hinting = 2;
pub static TTF_HINTING_NONE: TTF_Hinting = 3;

#[link(name="SDL2_ttf")]
extern "C" {
	pub fn TTF_Init() -> c_int;
	pub fn TTF_WasInit() -> c_int;
	pub fn TTF_Quit();
	pub fn TTF_OpenFont(file: *c_char, ptsize: c_int) -> *TTF_Font;
	pub fn TTF_OpenFontIndex(file: *c_char, ptsize: c_int, index: c_long)
	    -> *TTF_Font;
	pub fn TTF_CloseFont(font: *TTF_Font);

	pub fn TTF_GetFontStyle(font: *TTF_Font) -> TTF_StyleFlag;
	pub fn TTF_SetFontStyle(font: *TTF_Font, style: TTF_StyleFlag);
	pub fn TTF_GetFontOutline(font: *TTF_Font) -> c_int;
	pub fn TTF_SetFontOutline(font: *TTF_Font, outline: c_int);
	pub fn TTF_GetFontHinting(font: *TTF_Font) -> TTF_Hinting;
	pub fn TTF_SetFontHinting(font: *TTF_Font, hinting: TTF_Hinting);
	pub fn TTF_GetFontKerning(font: *TTF_Font) -> c_int;
	pub fn TTF_SetFontKerning(font: *TTF_Font, kerning: c_int);
	pub fn TTF_FontHeight(font: *TTF_Font) -> c_int;
	pub fn TTF_FontAscent(font: *TTF_Font) -> c_int;
	pub fn TTF_FontDescent(font: *TTF_Font) -> c_int;
	pub fn TTF_FontLineSkip(font: *TTF_Font) -> c_int;
	pub fn TTF_FontFaces(font: *TTF_Font) -> c_long;
	pub fn TTF_FontFaceIsFixedWidth(font: *TTF_Font) -> c_int;
	pub fn TTF_FontFaceFamilyName(font: *TTF_Font) -> *c_char;
	pub fn TTF_GlyphIsProvided(font: *TTF_Font, glyph: u16) -> c_int;
	pub fn TTF_GlyphMetrics(font: *TTF_Font, glyph: u16, minx: *mut c_int,
	    maxx: *mut c_int, miny: *mut c_int, maxy: *mut c_int,
	    advance: *mut c_int) -> c_int;
	pub fn TTF_SizeUTF8(font: *TTF_Font, text: *c_char, w: *mut c_int,
	    h: *mut c_int) -> c_int;

	pub fn TTF_RenderUTF8_Solid(font: *TTF_Font, text: *c_char,
	    fg: SDL_Color) -> *SDL_Surface;
	pub fn TTF_RenderUTF8_Shaded(font: *TTF_Font, text: *c_char,
	    fg: SDL_Color, bg: SDL_Color) -> *SDL_Surface;
	pub fn TTF_RenderUTF8_Blended(font: *TTF_Font, text: *c_char,
	    fg: SDL_Color) -> *SDL_Surface;
}
