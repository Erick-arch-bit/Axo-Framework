// Fase 1 — Conversión de objetos JavaScript → UiNode / StyleMap.
use std::sync::atomic::{AtomicU64, Ordering};
use rquickjs::{Array, Object, Value, convert::Coerced};
use crate::serde::{AlignItems, FlexDirection, JustifyContent, LengthValue, StyleMap, UiNode};

// Fase 1: contador simple para asignar id únicos.
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

// Fase 1: genera un id único para cada UiNode.
fn next_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

// Fase 1: parse "#rrggbb" / "#rrggbbaa" → [f32; 4].
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

// Fase 1: extrae un string JS (con coerción) desde un Value.
fn js_string(v: &Value) -> Option<std::string::String> {
    v.get::<Coerced<std::string::String>>().map(|c| c.0).ok()
}

// Fase 1: longitud JS → LengthValue (número → Pixels, string con % → Percent).
fn parse_js_length(v: Value) -> Option<LengthValue> {
    if let Some(n) = v.as_number() {
        return Some(LengthValue::Pixels(n as f32));
    }
    let s = js_string(&v)?;
    let s = s.trim();
    if let Some(stripped) = s.strip_suffix('%') {
        let num: f32 = stripped.trim().parse().ok()?;
        Some(LengthValue::Percent(num / 100.0))
    } else {
        let num: f32 = s.parse().ok()?;
        Some(LengthValue::Pixels(num))
    }
}

// Fase 1: número JS (o string numérico) → f32.
fn parse_js_float(v: Value) -> Option<f32> {
    if let Some(n) = v.as_number() {
        return Some(n as f32);
    }
    let s = js_string(&v)?;
    s.trim().parse().ok()
}

// Fase 1: color JS ("#rrggbb" / "#rrggbbaa") → [f32; 4].
fn parse_js_color(v: Value) -> Option<[f32; 4]> {
    let s = js_string(&v)?;
    parse_hex_color(&s)
}

// Fase 1: flexDirection JS → FlexDirection.
fn parse_js_flex_direction(v: Value) -> Option<FlexDirection> {
    match js_string(&v)?.as_str() {
        "row" => Some(FlexDirection::Row),
        "column" => Some(FlexDirection::Column),
        "rowReverse" | "row-reverse" => Some(FlexDirection::RowReverse),
        "columnReverse" | "column-reverse" => Some(FlexDirection::ColumnReverse),
        _ => None,
    }
}

// Fase 1: justifyContent JS → JustifyContent.
fn parse_js_justify_content(v: Value) -> Option<JustifyContent> {
    match js_string(&v)?.as_str() {
        "flexStart" | "flex-start" => Some(JustifyContent::FlexStart),
        "flexEnd" | "flex-end" => Some(JustifyContent::FlexEnd),
        "center" => Some(JustifyContent::Center),
        "spaceBetween" | "space-between" => Some(JustifyContent::SpaceBetween),
        "spaceAround" | "space-around" => Some(JustifyContent::SpaceAround),
        "spaceEvenly" | "space-evenly" => Some(JustifyContent::SpaceEvenly),
        _ => None,
    }
}

// Fase 1: alignItems JS → AlignItems.
fn parse_js_align_items(v: Value) -> Option<AlignItems> {
    match js_string(&v)?.as_str() {
        "flexStart" | "flex-start" => Some(AlignItems::FlexStart),
        "flexEnd" | "flex-end" => Some(AlignItems::FlexEnd),
        "center" => Some(AlignItems::Center),
        "stretch" => Some(AlignItems::Stretch),
        "baseline" => Some(AlignItems::Baseline),
        _ => None,
    }
}

// Fase 1: lee una propiedad opcional como Value (None si falta).
fn get_prop<'js>(obj: &Object<'js>, key: &str) -> Option<Value<'js>> {
    obj.get::<_, Option<Value<'js>>>(key).ok().flatten()
}

// Fase 1: convierte un objeto JS { type, content?, style?, children? } → UiNode (recursivo).
pub fn js_to_ui_node<'js>(value: Value<'js>) -> Result<UiNode, std::string::String> {
    let obj = Object::from_value(value).map_err(|e| e.to_string())?;

    // Fase 1: `type` obligatorio → node_type.
    let node_type: std::string::String = obj
        .get::<_, Option<Coerced<std::string::String>>>("type")
        .map_err(|e| e.to_string())?
        .map(|c| c.0)
        .ok_or_else(|| "missing 'type' in JS node".to_string())?;
    if node_type.is_empty() {
        return Err("missing 'type' in JS node".to_string());
    }

    // Fase 1: `content` opcional, default "".
    let content: std::string::String = obj
        .get::<_, Option<Coerced<std::string::String>>>("content")
        .map_err(|e| e.to_string())?
        .map(|c| c.0)
        .unwrap_or_default();

    // Fase 1: `style` parcial (solo las propiedades soportadas, resto se ignora).
    let mut style = StyleMap::new();
    if let Ok(Some(style_obj)) = obj.get::<_, Option<Object>>("style") {
        if let Some(v) = get_prop(&style_obj, "width") {
            if let Some(l) = parse_js_length(v) {
                style.width = Some(l);
            }
        }
        if let Some(v) = get_prop(&style_obj, "height") {
            if let Some(l) = parse_js_length(v) {
                style.height = Some(l);
            }
        }
        if let Some(v) = get_prop(&style_obj, "backgroundColor") {
            if let Some(c) = parse_js_color(v) {
                style.background_color = Some(c);
            }
        }
        if let Some(v) = get_prop(&style_obj, "flexDirection") {
            if let Some(f) = parse_js_flex_direction(v) {
                style.flex_direction = Some(f);
            }
        }
        if let Some(v) = get_prop(&style_obj, "justifyContent") {
            if let Some(j) = parse_js_justify_content(v) {
                style.justify_content = Some(j);
            }
        }
        if let Some(v) = get_prop(&style_obj, "alignItems") {
            if let Some(a) = parse_js_align_items(v) {
                style.align_items = Some(a);
            }
        }
        if let Some(v) = get_prop(&style_obj, "fontSize") {
            if let Some(f) = parse_js_float(v) {
                style.font_size = Some(f);
            }
        }
        if let Some(v) = get_prop(&style_obj, "color") {
            if let Some(c) = parse_js_color(v) {
                style.color = Some(c);
            }
        }
    }

    // Fase 1: `children` opcional, array recursivo, default vec![].
    let mut children = Vec::new();
    if let Ok(Some(arr)) = obj.get::<_, Option<Array>>("children") {
        for i in 0..arr.len() {
            let child_val: Value = arr.get(i).map_err(|e: rquickjs::Error| e.to_string())?;
            children.push(js_to_ui_node(child_val)?);
        }
    }

    // Fase 1: eventos se dejan como "" por ahora.
    Ok(UiNode {
        id: next_id(),
        node_type,
        content,
        style,
        children,
        on_change_text_id: std::string::String::new(),
    })
}
