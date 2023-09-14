use lsp_types::{RenameParams, TextEdit, WorkspaceEdit};
use std::collections::HashMap;
use store::Store;

pub fn provide_rename(store: &Store, params: RenameParams) -> Option<WorkspaceEdit> {
    let items = &store.get_items_from_position(
        params.text_document_position.position,
        &params.text_document_position.text_document.uri,
    );
    if items.len() != 1 {
        return None;
    }
    let item = items[0].read();

    let mut changes = HashMap::new();
    changes.insert(
        (*item.uri()).clone(),
        vec![TextEdit {
            range: item.v_range(),
            new_text: params.new_name.clone(),
        }],
    );
    for reference in item.references()? {
        let edit = TextEdit {
            range: reference.v_range,
            new_text: params.new_name.clone(),
        };
        if let Some(uri_changes) = changes.get_mut(store.path_interner.lookup(reference.file_id)) {
            uri_changes.push(edit)
        } else {
            changes.insert(
                store.path_interner.lookup(reference.file_id).clone(),
                vec![edit],
            );
        }
    }

    Some(WorkspaceEdit {
        changes: Some(changes),
        ..Default::default()
    })
}
