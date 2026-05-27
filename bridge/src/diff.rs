use crate::serde::UiNode;

#[derive(Debug)]
pub enum Patch {
    AddNode { parent_id: u64, node: UiNode },
    RemoveNode { id: u64 },
    UpdateProps { id: u64, props: std::collections::HashMap<String, String> },
    Reorder { parent_id: u64, order: Vec<u64> },
}

pub fn diff_trees(old: &[UiNode], new: &[UiNode]) -> Vec<Patch> {
    let mut patches = Vec::new();

    for new_node in new {
        let exists = old.iter().any(|n| n.id == new_node.id);
        if !exists {
            patches.push(Patch::AddNode {
                parent_id: 0,
                node: new_node.clone(),
            });
        } else if let Some(old_node) = old.iter().find(|n| n.id == new_node.id) {
            if old_node.props != new_node.props {
                patches.push(Patch::UpdateProps {
                    id: new_node.id,
                    props: new_node.props.clone(),
                });
            }
        }
    }

    for old_node in old {
        if !new.iter().any(|n| n.id == old_node.id) {
            patches.push(Patch::RemoveNode { id: old_node.id });
        }
    }

    patches
}
