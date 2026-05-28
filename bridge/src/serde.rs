use mlua::prelude::*;

#[derive(Debug, Clone)]
pub struct UiNode {
    pub id: u64,
    pub node_type: String,
    pub content: String,
    pub style: StyleMap,
    pub children: Vec<UiNode>,
}

#[derive(Debug, Clone)]
pub struct StyleMap {
    pub width: Option<LengthValue>,
    pub height: Option<LengthValue>,
    pub background_color: Option<[f32; 4]>,
    pub margin: Option<RectAuto>,
    pub padding: Option<Rect>,
    pub flex_direction: Option<FlexDirection>,
    pub justify_content: Option<JustifyContent>,
    pub align_items: Option<AlignItems>,
    pub font_size: Option<f32>,
    pub color: Option<[f32; 4]>,
    pub on_click_id: String,
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

impl Default for StyleMap {
    fn default() -> Self {
        Self::new()
    }
}

impl StyleMap {
    pub fn new() -> Self {
        StyleMap {
            width: None, height: None, background_color: None,
            margin: None, padding: None,
            flex_direction: None, justify_content: None, align_items: None,
            font_size: None, color: None,
            on_click_id: String::new(),
        }
    }
}

fn parse_length(s: &str) -> Option<LengthValue> {
    let s = s.trim();
    if let Some(stripped) = s.strip_suffix('%') {
        let num: f32 = stripped.trim().parse().ok()?;
        Some(LengthValue::Percent(num / 100.0))
    } else {
        let num: f32 = s.parse().ok()?;
        Some(LengthValue::Pixels(num))
    }
}

fn parse_hex_color(s: &str) -> Option<[f32; 4]> {
    let s = s.trim().strip_prefix('#')?;
    if s.len() == 6 {
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some([r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0])
    } else if s.len() == 8 {
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        let a = u8::from_str_radix(&s[6..8], 16).ok()?;
        Some([r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0])
    } else {
        None
    }
}

fn parse_rect(s: &str) -> Option<Rect> {
    let parts: Vec<f32> = s.split_whitespace().filter_map(|p| p.parse().ok()).collect();
    match parts.len() {
        1 => Some(Rect { left: parts[0], right: parts[0], top: parts[0], bottom: parts[0] }),
        2 => Some(Rect { left: parts[1], right: parts[1], top: parts[0], bottom: parts[0] }),
        4 => Some(Rect { left: parts[3], right: parts[1], top: parts[0], bottom: parts[2] }),
        _ => None,
    }
}

fn parse_rect_auto(s: &str) -> Option<RectAuto> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    let parse = |s: &str| -> AutoLength {
        if s == "auto" { AutoLength::Auto }
        else if let Some(stripped) = s.strip_suffix('%') { AutoLength::Percent(stripped.parse().unwrap_or(0.0) / 100.0) }
        else { AutoLength::Length(s.parse().unwrap_or(0.0)) }
    };
    match parts.len() {
        1 => { let v = parse(parts[0]); Some(RectAuto { left: v.clone(), right: v.clone(), top: v.clone(), bottom: v }) }
        2 => { let v = parse(parts[0]); let h = parse(parts[1]); Some(RectAuto { top: v.clone(), bottom: v, left: h.clone(), right: h }) }
        4 => Some(RectAuto { top: parse(parts[0]), right: parse(parts[1]), bottom: parse(parts[2]), left: parse(parts[3]) }),
        _ => None,
    }
}

fn parse_flex_direction(s: &str) -> Option<FlexDirection> {
    match s {
        "row" => Some(FlexDirection::Row),
        "column" => Some(FlexDirection::Column),
        "rowReverse" | "row-reverse" => Some(FlexDirection::RowReverse),
        "columnReverse" | "column-reverse" => Some(FlexDirection::ColumnReverse),
        _ => None,
    }
}

fn parse_justify_content(s: &str) -> Option<JustifyContent> {
    match s {
        "flexStart" | "flex-start" => Some(JustifyContent::FlexStart),
        "flexEnd" | "flex-end" => Some(JustifyContent::FlexEnd),
        "center" => Some(JustifyContent::Center),
        "spaceBetween" | "space-between" => Some(JustifyContent::SpaceBetween),
        "spaceAround" | "space-around" => Some(JustifyContent::SpaceAround),
        "spaceEvenly" | "space-evenly" => Some(JustifyContent::SpaceEvenly),
        _ => None,
    }
}

fn parse_align_items(s: &str) -> Option<AlignItems> {
    match s {
        "flexStart" | "flex-start" => Some(AlignItems::FlexStart),
        "flexEnd" | "flex-end" => Some(AlignItems::FlexEnd),
        "center" => Some(AlignItems::Center),
        "stretch" => Some(AlignItems::Stretch),
        "baseline" => Some(AlignItems::Baseline),
        _ => None,
    }
}

fn style_get_string(table: &LuaTable, key: &str) -> Option<String> {
    let v: LuaValue = table.get(key).ok()?;
    if let Ok(s) = v.to_string() { Some(s.to_string()) } else { None }
}

fn parse_style(table: &LuaTable) -> LuaResult<StyleMap> {
    let mut style = StyleMap::new();

    if let Some(s) = style_get_string(table, "width") { style.width = parse_length(&s); }
    if let Some(s) = style_get_string(table, "height") { style.height = parse_length(&s); }
    if let Some(s) = style_get_string(table, "backgroundColor") { style.background_color = parse_hex_color(&s); }
    if let Some(s) = style_get_string(table, "margin") { style.margin = parse_rect_auto(&s); }
    if let Some(s) = style_get_string(table, "padding") { style.padding = parse_rect(&s); }
    if let Some(s) = style_get_string(table, "flexDirection") { style.flex_direction = parse_flex_direction(&s); }
    if let Some(s) = style_get_string(table, "justifyContent") { style.justify_content = parse_justify_content(&s); }
    if let Some(s) = style_get_string(table, "alignItems") { style.align_items = parse_align_items(&s); }
    if let Some(s) = style_get_string(table, "color") { style.color = parse_hex_color(&s); }
    if let Some(s) = style_get_string(table, "fontSize") { if let Ok(n) = s.parse() { style.font_size = Some(n); } }

    Ok(style)
}

pub fn table_to_ui_node(table: &LuaTable, lua: &Lua) -> LuaResult<UiNode> {
    let node_type: String = table.get("type").unwrap_or_default();
    let content: String = table.get("content").unwrap_or_default();

    let style_table: LuaTable = match table.get("style") {
        Ok(t) => t,
        Err(_) => lua.create_table()?,
    };
    let mut style = parse_style(&style_table)?;

    // Parse onClick as a string function name (e.g., "handleClick")
    if let Ok(on_click) = table.get::<LuaValue>("onClick") {
        match on_click {
            LuaValue::String(s) => {
                style.on_click_id = s.to_string_lossy().to_string();
            }
            LuaValue::Function(f) => {
                // Store function reference by name and store the name
                let name = format!("__axo_cb_{}", f.to_pointer() as u64);
                let globals = lua.globals();
                if let Ok(callbacks) = globals.get::<LuaTable>("_AXO_CALLBACKS") {
                    callbacks.set(name.as_str(), f)?;
                }
                style.on_click_id = name;
            }
            _ => {}
        }
    }

    let children_raw: LuaTable = match table.get("children") {
        Ok(t) => t,
        Err(_) => lua.create_table()?,
    };

    let mut children = Vec::new();
    for pair in children_raw.pairs::<LuaValue, LuaTable>() {
        let (_, child_table) = pair?;
        children.push(table_to_ui_node(&child_table, lua)?);
    }

    Ok(UiNode { id: 0, node_type, content, style, children })
}
