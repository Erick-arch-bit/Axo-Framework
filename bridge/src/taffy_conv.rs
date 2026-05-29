use axo_core::layout::Engine;
use axo_core::renderer::Rect;
use taffy::prelude::*;

use crate::serde::{self, UiNode, AlignItems, AlignSelf, AlignContent, AutoLength, FlexDirection, FlexWrap, JustifyContent, LengthValue, PositionType};

fn map_length(v: &Option<LengthValue>) -> Dimension {
    match v {
        Some(LengthValue::Pixels(n)) => Dimension::Length(*n),
        Some(LengthValue::Percent(p)) => Dimension::Percent(*p),
        None => Dimension::Auto,
    }
}

fn map_margin(m: &Option<serde::RectAuto>) -> taffy::Rect<LengthPercentageAuto> {
    match m {
        Some(r) => {
            let map = |a: &AutoLength| match a {
                AutoLength::Length(n) => LengthPercentageAuto::Length(*n),
                AutoLength::Percent(p) => LengthPercentageAuto::Percent(*p),
                AutoLength::Auto => LengthPercentageAuto::Auto,
            };
            taffy::Rect { left: map(&r.left), right: map(&r.right), top: map(&r.top), bottom: map(&r.bottom) }
        }
        None => taffy::Rect { left: LengthPercentageAuto::Length(0.0), right: LengthPercentageAuto::Length(0.0), top: LengthPercentageAuto::Length(0.0), bottom: LengthPercentageAuto::Length(0.0) },
    }
}

fn map_padding(p: &Option<serde::Rect>) -> taffy::Rect<LengthPercentage> {
    match p {
        Some(r) => taffy::Rect { left: LengthPercentage::Length(r.left), right: LengthPercentage::Length(r.right), top: LengthPercentage::Length(r.top), bottom: LengthPercentage::Length(r.bottom) },
        None => taffy::Rect { left: LengthPercentage::Length(0.0), right: LengthPercentage::Length(0.0), top: LengthPercentage::Length(0.0), bottom: LengthPercentage::Length(0.0) },
    }
}

fn map_flex_direction(d: &Option<FlexDirection>) -> taffy::FlexDirection {
    match d {
        Some(FlexDirection::Row) => taffy::FlexDirection::Row,
        Some(FlexDirection::Column) => taffy::FlexDirection::Column,
        Some(FlexDirection::RowReverse) => taffy::FlexDirection::RowReverse,
        Some(FlexDirection::ColumnReverse) => taffy::FlexDirection::ColumnReverse,
        None => taffy::FlexDirection::Column,
    }
}

fn map_justify_content(j: &Option<JustifyContent>) -> Option<taffy::JustifyContent> {
    j.as_ref().map(|j| match j {
        JustifyContent::FlexStart => taffy::JustifyContent::FlexStart,
        JustifyContent::FlexEnd => taffy::JustifyContent::FlexEnd,
        JustifyContent::Center => taffy::JustifyContent::Center,
        JustifyContent::SpaceBetween => taffy::JustifyContent::SpaceBetween,
        JustifyContent::SpaceAround => taffy::JustifyContent::SpaceAround,
        JustifyContent::SpaceEvenly => taffy::JustifyContent::SpaceEvenly,
    })
}

fn map_align_items(a: &Option<AlignItems>) -> Option<taffy::AlignItems> {
    a.as_ref().map(|a| match a {
        AlignItems::FlexStart => taffy::AlignItems::FlexStart,
        AlignItems::FlexEnd => taffy::AlignItems::FlexEnd,
        AlignItems::Center => taffy::AlignItems::Center,
        AlignItems::Stretch => taffy::AlignItems::Stretch,
        AlignItems::Baseline => taffy::AlignItems::Baseline,
    })
}

fn map_align_self(a: &Option<AlignSelf>) -> Option<taffy::AlignItems> {
    match a.as_ref()? {
        AlignSelf::Auto => None,
        AlignSelf::FlexStart => Some(taffy::AlignItems::FlexStart),
        AlignSelf::FlexEnd => Some(taffy::AlignItems::FlexEnd),
        AlignSelf::Center => Some(taffy::AlignItems::Center),
        AlignSelf::Stretch => Some(taffy::AlignItems::Stretch),
        AlignSelf::Baseline => Some(taffy::AlignItems::Baseline),
    }
}

fn map_align_content(a: &Option<AlignContent>) -> Option<taffy::AlignContent> {
    a.as_ref().map(|a| match a {
        AlignContent::FlexStart => taffy::AlignContent::FlexStart,
        AlignContent::FlexEnd => taffy::AlignContent::FlexEnd,
        AlignContent::Center => taffy::AlignContent::Center,
        AlignContent::Stretch => taffy::AlignContent::Stretch,
        AlignContent::SpaceBetween => taffy::AlignContent::SpaceBetween,
        AlignContent::SpaceAround => taffy::AlignContent::SpaceAround,
    })
}

fn map_flex_wrap(w: &Option<FlexWrap>) -> taffy::FlexWrap {
    match w {
        Some(FlexWrap::Wrap) => taffy::FlexWrap::Wrap,
        Some(FlexWrap::WrapReverse) => taffy::FlexWrap::WrapReverse,
        _ => taffy::FlexWrap::NoWrap,
    }
}

fn map_position(p: &Option<PositionType>) -> taffy::Position {
    match p {
        Some(PositionType::Absolute) => taffy::Position::Absolute,
        _ => taffy::Position::Relative,
    }
}

fn map_inset(s: &Option<LengthValue>) -> taffy::LengthPercentageAuto {
    match s {
        Some(LengthValue::Pixels(n)) => taffy::LengthPercentageAuto::Length(*n),
        Some(LengthValue::Percent(p)) => taffy::LengthPercentageAuto::Percent(*p),
        None => taffy::LengthPercentageAuto::Auto,
    }
}

fn to_style(node: &UiNode) -> Style {
    let s = &node.style;
    Style {
        display: taffy::Display::Flex,
        flex_direction: map_flex_direction(&s.flex_direction),
        align_items: map_align_items(&s.align_items),
        align_self: map_align_self(&s.align_self),
        align_content: map_align_content(&s.align_content),
        justify_content: map_justify_content(&s.justify_content),
        flex_wrap: map_flex_wrap(&s.flex_wrap),
        flex_grow: s.flex_grow.unwrap_or(0.0),
        flex_shrink: s.flex_shrink.unwrap_or(1.0),
        gap: taffy::Size { width: LengthPercentage::Length(s.gap.unwrap_or(0.0)), height: LengthPercentage::Length(0.0) },
        position: map_position(&s.position),
        inset: taffy::Rect {
            top: map_inset(&s.top),
            left: map_inset(&s.left),
            right: map_inset(&s.right),
            bottom: map_inset(&s.bottom),
        },
        size: Size { width: map_length(&s.width), height: map_length(&s.height) },
        min_size: Size { width: map_length(&s.min_width), height: map_length(&s.min_height) },
        max_size: Size { width: map_length(&s.max_width), height: map_length(&s.max_height) },
        margin: map_margin(&s.margin),
        padding: map_padding(&s.padding),
        ..Default::default()
    }
}

fn convert_tree(engine: &mut Engine, node: &UiNode, next_id: &mut u64) -> (taffy::NodeId, u64) {
    let id = *next_id;
    *next_id += 1;

    let node_type = &node.node_type;
    let color = node.style.background_color.unwrap_or(if node.children.is_empty() { [0.3, 0.3, 0.3, 1.0] } else { [0.0, 0.0, 0.0, 0.0] });
    let _text_color = node.style.color.unwrap_or([1.0, 1.0, 1.0, 1.0]);

    let taffy_id = if node.children.is_empty() {
        engine.create_leaf(to_style(node), color, node_type)
    } else {
        let mut child_ids = Vec::new();
        for child in &node.children {
            child_ids.push(convert_tree(engine, child, next_id).0);
        }
        engine.create_container(to_style(node), &child_ids, color, node_type)
    };

    (taffy_id, id)
}

fn collect_node_data(node: &UiNode, map: &mut std::collections::HashMap<u64, UiNodeMeta>, next_id: &mut u64) {
    let id = *next_id;
    *next_id += 1;
    let image_source = if node.node_type == "Image" && !node.content.is_empty() {
        Some(node.content.clone())
    } else {
        None
    };
    map.insert(id, UiNodeMeta {
        border_radius: node.style.border_radius.unwrap_or(0.0),
        on_click_id: node.style.on_click_id.clone(),
        on_change_text_id: node.on_change_text_id.clone(),
        text_content: node.content.clone(),
        font_size: node.style.font_size.unwrap_or(16.0),
        text_color: node.style.color.unwrap_or([1.0, 1.0, 1.0, 1.0]),
        image_source,
        hover_background_color: node.style.hover_background_color,
        active_background_color: node.style.active_background_color,
        hover_color: node.style.hover_color,
        active_color: node.style.active_color,
        disabled: node.style.disabled,
    });
    for child in &node.children {
        collect_node_data(child, map, next_id);
    }
}

#[derive(Clone)]
struct UiNodeMeta {
    border_radius: f32,
    on_click_id: String,
    on_change_text_id: String,
    text_content: String,
    font_size: f32,
    text_color: [f32; 4],
    image_source: Option<String>,
    hover_background_color: Option<[f32; 4]>,
    active_background_color: Option<[f32; 4]>,
    hover_color: Option<[f32; 4]>,
    active_color: Option<[f32; 4]>,
    disabled: bool,
}

pub fn build_rects(
    engine: &mut Engine,
    root: &UiNode,
    viewport_width: f32,
    viewport_height: f32,
    scroll_offsets: &std::collections::HashMap<u64, (f32, f32)>,
) -> Vec<Rect> {
    let mut next_id = 1u64;
    let (root_taffy, _) = convert_tree(engine, root, &mut next_id);

    let mut meta_map = std::collections::HashMap::new();
    collect_node_data(root, &mut meta_map, &mut 1u64);

    engine.compute(root_taffy, viewport_width, viewport_height);

    let mut rects = Vec::new();
    let mut node_id = 1u64;
    let mut scroll_stack: Vec<ScrollFrame> = Vec::new();

    engine.visit(root_taffy, &mut |tid, layout| {
            let color = engine.get_color(tid);
            if color[3] > 0.0 {
                let node_type = engine.get_type(tid);
                let meta = meta_map.get(&node_id).cloned().unwrap_or(UiNodeMeta {
                    border_radius: 0.0,
                    on_click_id: String::new(),
                    on_change_text_id: String::new(),
                    text_content: String::new(),
                    font_size: 16.0,
                    text_color: [1.0, 1.0, 1.0, 1.0],
                    image_source: None,
                    hover_background_color: None,
                    active_background_color: None,
                    hover_color: None,
                    active_color: None,
                    disabled: false,
                });

                // Check if this node is a ScrollView
                let is_scroll = node_type == "ScrollView";

                // Determine clip rect from active scroll container
                let clip_rect = scroll_stack.last().map(|sf| {
                    [sf.rect_x, sf.rect_y, sf.rect_w, sf.rect_h]
                });

                // Get scroll offset (from this node if it's a ScrollView, or from parent)
                let (scroll_ox, scroll_oy) = if is_scroll {
                    scroll_offsets.get(&node_id).copied().unwrap_or((0.0, 0.0))
                } else {
                    scroll_stack.last().map(|sf| sf.offset).unwrap_or((0.0, 0.0))
                };

                rects.push(Rect {
                    id: node_id,
                    x: layout.location.x,
                    y: layout.location.y,
                    w: layout.size.width,
                    h: layout.size.height,
                    color,
                    node_type,
                    on_click_id: meta.on_click_id,
                    on_change_text_id: meta.on_change_text_id,
                    text_content: meta.text_content,
                    font_size: meta.font_size,
                    text_color: meta.text_color,
                    scroll_offset_x: scroll_ox,
                    scroll_offset_y: scroll_oy,
                    clip_rect,
                    image_source: meta.image_source.clone(),
                    hover_color: meta.hover_background_color,
                    active_color: meta.active_background_color,
                    hover_text_color: meta.hover_color,
                    active_text_color: meta.active_color,
                    disabled: meta.disabled,
                    border_radius: meta.border_radius,
                });

                // If this is a ScrollView, push a scroll frame for its children
                if is_scroll {
                    scroll_stack.push(ScrollFrame {
                        rect_x: layout.location.x - scroll_ox,
                        rect_y: layout.location.y - scroll_oy,
                        rect_w: layout.size.width,
                        rect_h: layout.size.height,
                        offset: (scroll_ox, scroll_oy),
                    });
                }
            }
        node_id += 1;
    });

    rects
}

struct ScrollFrame {
    rect_x: f32,
    rect_y: f32,
    rect_w: f32,
    rect_h: f32,
    offset: (f32, f32),
}

/// Simplified build without scroll state (for tests / simple usage)
pub fn build_rects_simple(engine: &mut Engine, root: &UiNode, viewport_width: f32, viewport_height: f32) -> Vec<Rect> {
    let empty = std::collections::HashMap::new();
    build_rects(engine, root, viewport_width, viewport_height, &empty)
}
