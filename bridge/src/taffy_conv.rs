use lumina_core::layout::Engine;
use lumina_core::renderer::Rect;
use taffy::prelude::*;

use crate::serde::{self, UiNode, AlignItems, AutoLength, FlexDirection, JustifyContent, LengthValue};

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

fn to_style(node: &UiNode) -> Style {
    let s = &node.style;
    Style {
        display: taffy::Display::Flex,
        flex_direction: map_flex_direction(&s.flex_direction),
        align_items: map_align_items(&s.align_items),
        justify_content: map_justify_content(&s.justify_content),
        size: Size { width: map_length(&s.width), height: map_length(&s.height) },
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
    map.insert(id, UiNodeMeta {
        on_click_id: node.style.on_click_id.clone(),
        text_content: node.content.clone(),
        font_size: node.style.font_size.unwrap_or(16.0),
        text_color: node.style.color.unwrap_or([1.0, 1.0, 1.0, 1.0]),
    });
    for child in &node.children {
        collect_node_data(child, map, next_id);
    }
}

#[derive(Clone)]
struct UiNodeMeta {
    on_click_id: String,
    text_content: String,
    font_size: f32,
    text_color: [f32; 4],
}

pub fn build_rects(engine: &mut Engine, root: &UiNode, viewport_width: f32, viewport_height: f32) -> Vec<Rect> {
    let mut next_id = 1u64;
    let (root_taffy, _) = convert_tree(engine, root, &mut next_id);

    let mut meta_map = std::collections::HashMap::new();
    collect_node_data(root, &mut meta_map, &mut 1u64);

    engine.compute(root_taffy, viewport_width, viewport_height);

    let mut rects = Vec::new();
    let mut node_id = 1u64;
    engine.visit(root_taffy, &mut |tid, layout| {
        let color = engine.get_color(tid);
        if color[3] > 0.0 {
            let node_type = engine.get_type(tid);
            let meta = meta_map.get(&node_id).cloned().unwrap_or(UiNodeMeta {
                on_click_id: String::new(),
                text_content: String::new(),
                font_size: 16.0,
                text_color: [1.0, 1.0, 1.0, 1.0],
            });
            rects.push(Rect {
                id: node_id,
                x: layout.location.x,
                y: layout.location.y,
                w: layout.size.width,
                h: layout.size.height,
                color,
                node_type,
                on_click_id: meta.on_click_id,
                text_content: meta.text_content,
                font_size: meta.font_size,
                text_color: meta.text_color,
            });
        }
        node_id += 1;
    });

    rects
}
