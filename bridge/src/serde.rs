#[derive(Debug, Clone)]
pub struct UiNode {
    pub id: u64,
    pub node_type: String,
    pub content: String,
    pub style: StyleMap,
    pub children: Vec<UiNode>,
    pub on_change_text_id: String,
}

#[derive(Debug, Clone)]
pub struct StyleMap {
    pub width: Option<LengthValue>,
    pub height: Option<LengthValue>,
    pub min_width: Option<LengthValue>,
    pub max_width: Option<LengthValue>,
    pub min_height: Option<LengthValue>,
    pub max_height: Option<LengthValue>,
    pub background_color: Option<[f32; 4]>,
    pub margin: Option<RectAuto>,
    pub padding: Option<Rect>,
    pub flex_direction: Option<FlexDirection>,
    pub justify_content: Option<JustifyContent>,
    pub align_items: Option<AlignItems>,
    pub align_self: Option<AlignSelf>,
    pub align_content: Option<AlignContent>,
    pub flex_wrap: Option<FlexWrap>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub gap: Option<f32>,
    pub position: Option<PositionType>,
    pub top: Option<LengthValue>,
    pub left: Option<LengthValue>,
    pub right: Option<LengthValue>,
    pub bottom: Option<LengthValue>,
    pub border_width: Option<f32>,
    pub border_color: Option<[f32; 4]>,
    pub font_size: Option<f32>,
    pub color: Option<[f32; 4]>,
    pub border_radius: Option<f32>,
    pub on_click_id: String,
    pub hover_background_color: Option<[f32; 4]>,
    pub active_background_color: Option<[f32; 4]>,
    pub hover_color: Option<[f32; 4]>,
    pub active_color: Option<[f32; 4]>,
    pub disabled: bool,
}

#[derive(Debug, Clone)]
pub enum LengthValue {
    Pixels(f32),
    Percent(f32),
}

#[derive(Debug, Clone)]
pub struct RectAuto {
    pub left: AutoLength,
    pub right: AutoLength,
    pub top: AutoLength,
    pub bottom: AutoLength,
}

#[derive(Debug, Clone)]
pub enum AutoLength {
    Length(f32),
    Percent(f32),
    Auto,
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Debug, Clone)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Debug, Clone)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    Baseline,
}

#[derive(Debug, Clone)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Debug, Clone)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    Baseline,
}

#[derive(Debug, Clone)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

#[derive(Debug, Clone)]
pub enum PositionType {
    Relative,
    Absolute,
}

impl Default for StyleMap {
    fn default() -> Self {
        Self::new()
    }
}

impl StyleMap {
    pub fn new() -> Self {
        StyleMap {
            width: None, height: None,
            min_width: None, max_width: None, min_height: None, max_height: None,
            background_color: None,
            margin: None, padding: None,
            flex_direction: None, justify_content: None, align_items: None,
            align_self: None, align_content: None, flex_wrap: None,
            flex_grow: None, flex_shrink: None, gap: None,
            position: None, top: None, left: None, right: None, bottom: None,
            border_width: None, border_color: None,
            font_size: None, color: None,
            border_radius: None,
            on_click_id: String::new(),
            hover_background_color: None,
            active_background_color: None,
            hover_color: None,
            active_color: None,
            disabled: false,
        }
    }
}