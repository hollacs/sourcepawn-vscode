use anyhow::Context;
use parking_lot::RwLock;
use std::sync::Arc;
use syntax::{property_item::PropertyItem, utils::ts_range_to_lsp_range, SPItem};
use tree_sitter::Node;

use crate::Parser;

impl<'a> Parser<'a> {
    pub fn parse_property(
        &mut self,
        node: &mut Node,
        parent: Arc<RwLock<SPItem>>,
    ) -> anyhow::Result<()> {
        let name_node = node
            .child_by_field_name("name")
            .context("Property name is empty.")?;
        let name = name_node.utf8_text(self.source.as_bytes())?.to_string();
        let type_node = node
            .child_by_field_name("type")
            .context("Property type is empty.")?;
        let type_ = type_node.utf8_text(self.source.as_bytes())?.to_string();

        let range = ts_range_to_lsp_range(&name_node.range());
        let full_range = ts_range_to_lsp_range(&node.range());
        let property_item = PropertyItem {
            name,
            range,
            v_range: self.build_v_range(&range),
            full_range,
            v_full_range: self.build_v_range(&full_range),
            type_,
            description: self
                .find_doc(node.start_position().row, false)
                .unwrap_or_default(),
            uri: self.uri.clone(),
            file_id: self.file_id,
            references: vec![],
            parent: Arc::downgrade(&parent),
        };

        let property_item = Arc::new(RwLock::new(SPItem::Property(property_item)));
        parent.write().push_child(property_item.clone());
        self.declarations
            .insert(property_item.clone().read().key(), property_item);

        // TODO: Add getter and setter parsing.
        Ok(())
    }
}
