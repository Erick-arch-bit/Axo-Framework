use ab_glyph::{Font, FontArc, Glyph, PxScale, ScaleFont};

pub struct TextRenderer {
    font: FontArc,
}

#[derive(Debug, Clone)]
pub struct GlyphQuad {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl TextRenderer {
    pub fn new() -> Self {
        let font_data: &[u8] = include_bytes!("../../../assets/LiberationSans-Regular.ttf");
        let font = FontArc::try_from_slice(font_data)
            .expect("Failed to load LiberationSans font");
        Self { font }
    }

    /// Measure a string and return (width, height) in pixels
    pub fn measure(&self, text: &str, font_size: f32, max_width: f32) -> (f32, f32) {
        let px_size = PxScale::from(font_size);
        let scaled = self.font.as_scaled(px_size);

        let mut max_x: f32 = 0.0;
        let mut x: f32 = 0.0;
        let mut y: f32 = scaled.height();
        let line_height: f32 = scaled.height() + scaled.line_gap();

        for ch in text.chars() {
            if ch == '\n' {
                max_x = max_x.max(x);
                x = 0.0;
                y += line_height;
                continue;
            }
            if x > max_width && max_width > 0.0 {
                max_x = max_x.max(x);
                x = 0.0;
                y += line_height;
            }

            let advance = scaled.h_advance(self.font.glyph_id(ch));
            x += advance;
        }
        max_x = max_x.max(x);

        (max_x, y)
    }

    /// Generate glyph quads for a text string, positioned at (0,0) baseline
    pub fn layout(&self, text: &str, font_size: f32, max_width: f32) -> Vec<GlyphQuad> {
        let px_size = PxScale::from(font_size);
        let scaled = self.font.as_scaled(px_size);

        let mut quads = Vec::new();
        let mut x: f32 = 0.0;
        let mut y: f32 = scaled.height() - scaled.ascent();
        let line_height: f32 = scaled.height() + scaled.line_gap();

        for ch in text.chars() {
            if ch == '\n' {
                x = 0.0;
                y += line_height;
                continue;
            }
            if x > max_width && max_width > 0.0 {
                x = 0.0;
                y += line_height;
            }

            let glyph_id = self.font.glyph_id(ch);
            let glyph = Glyph {
                id: glyph_id,
                scale: px_size,
                position: ab_glyph::Point { x, y },
            };

            if let Some(outline) = self.font.outline_glyph(glyph) {
                let bounds = outline.px_bounds();
                quads.push(GlyphQuad {
                    x: bounds.min.x,
                    y: bounds.min.y,
                    w: bounds.width(),
                    h: bounds.height(),
                });
            }

            let advance = scaled.h_advance(glyph_id);
            x += advance;
        }

        quads
    }
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}
