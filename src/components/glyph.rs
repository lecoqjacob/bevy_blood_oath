use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub struct Glyph {
    pub glyph: FontCharType,
    pub color: ColorPair,
}
