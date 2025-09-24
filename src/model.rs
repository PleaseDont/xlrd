use super::record::{
    font,
    xf::{self, HorizAlign, Pattern, VertAlign},
};
use std::collections::HashMap;
use umya_spreadsheet::{HorizontalAlignmentValues, PatternValues, VerticalAlignmentValues};

#[derive(Debug)]
pub enum Value<'a> {
    String(&'a str),
    Number(f64),
}

#[derive(Debug, Default)]
pub struct Global {
    pub date1904: bool,
    pub palette: Option<Vec<String>>,
    pub fonts: Vec<font::Data>,
    pub formats: HashMap<u32, String>,
    pub xfs: Vec<xf::Data>,
}

impl From<HorizAlign> for HorizontalAlignmentValues {
    fn from(align: HorizAlign) -> HorizontalAlignmentValues {
        match align {
            HorizAlign::General => HorizontalAlignmentValues::General,
            HorizAlign::Left => HorizontalAlignmentValues::Left,
            HorizAlign::Center => HorizontalAlignmentValues::Center,
            HorizAlign::Right => HorizontalAlignmentValues::Right,
            HorizAlign::Fill => HorizontalAlignmentValues::Fill,
            HorizAlign::Justify => HorizontalAlignmentValues::Justify,
            HorizAlign::CenterContinuous => HorizontalAlignmentValues::CenterContinuous,
            HorizAlign::Distributed => HorizontalAlignmentValues::Distributed,
        }
    }
}

impl From<VertAlign> for VerticalAlignmentValues {
    fn from(align: VertAlign) -> VerticalAlignmentValues {
        match align {
            VertAlign::Top => VerticalAlignmentValues::Top,
            VertAlign::Center => VerticalAlignmentValues::Center,
            VertAlign::Bottom => VerticalAlignmentValues::Bottom,
            VertAlign::Justify => VerticalAlignmentValues::Justify,
            VertAlign::Distributed => VerticalAlignmentValues::Distributed,
        }
    }
}

impl From<Pattern> for PatternValues {
    fn from(pattern: Pattern) -> PatternValues {
        match pattern {
            Pattern::None => PatternValues::None,
            Pattern::Solid => PatternValues::Solid,
            Pattern::MediumGray => PatternValues::MediumGray,
            Pattern::DarkGray => PatternValues::DarkGray,
            Pattern::LightGray => PatternValues::LightGray,
            Pattern::DarkHorizontal => PatternValues::DarkHorizontal,
            Pattern::DarkVertical => PatternValues::DarkVertical,
            Pattern::DarkDown => PatternValues::DarkDown,
            Pattern::DarkUp => PatternValues::DarkUp,
            Pattern::DarkGrid => PatternValues::DarkGrid,
            Pattern::DarkTrellis => PatternValues::DarkTrellis,
            Pattern::LightHorizontal => PatternValues::LightHorizontal,
            Pattern::LightVertical => PatternValues::LightVertical,
            Pattern::LightDown => PatternValues::LightDown,
            Pattern::LightUp => PatternValues::LightUp,
            Pattern::LightGrid => PatternValues::LightGrid,
            Pattern::LightTrellis => PatternValues::LightTrellis,
            Pattern::Gray125 => PatternValues::Gray125,
            Pattern::Gray0625 => PatternValues::Gray0625,
        }
    }
}
