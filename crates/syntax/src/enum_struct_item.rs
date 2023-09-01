use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, CompletionParams,
    DocumentSymbol, GotoDefinitionParams, Hover, HoverContents, HoverParams, LanguageString,
    LocationLink, MarkedString, Range, SymbolKind, SymbolTag, Url,
};
use parking_lot::RwLock;
use std::sync::Arc;

use crate::{description::Description, uri_to_file_name, FileId, Reference, SPItem};

#[derive(Debug, Clone)]
/// SPItem representation of a SourcePawn enum struct.
pub struct EnumStructItem {
    /// Name of the enum struct.
    pub name: String,

    /// Range of the name of the enum struct.
    pub range: Range,

    /// User visible range of the name of the enum struct.
    pub v_range: Range,

    /// Range of the whole enum struct, including its block.
    pub full_range: Range,

    /// User visible range of the whole enum struct, including its block.
    pub v_full_range: Range,

    /// Description of the enum struct.
    pub description: Description,

    /// Uri of the file where the enum struct is declared.
    pub uri: Arc<Url>,

    /// [FileId](FileId) of the file where the enum struct is declared.
    pub file_id: FileId,

    /// References to this enum struct.
    pub references: Vec<Reference>,

    /// Children ([FunctionItem](super::function_item::FunctionItem),
    /// [VariableItem](super::variable_item::VariableItem)) of this enum struct.
    pub children: Vec<Arc<RwLock<SPItem>>>,
}

impl EnumStructItem {
    /// Return a vector of [CompletionItem](lsp_types::CompletionItem) from an [EnumStructItem] and its children.
    ///
    /// # Arguments
    ///
    /// * `params` - [CompletionParams](lsp_types::CompletionParams) of the request.
    /// * `request_method` - Whether we are requesting method completions or not.
    pub fn to_completions(
        &self,
        params: &CompletionParams,
        request_method: bool,
    ) -> Vec<CompletionItem> {
        let mut res = vec![];
        res.push(CompletionItem {
            label: self.name.to_string(),
            kind: Some(CompletionItemKind::STRUCT),
            label_details: Some(CompletionItemLabelDetails {
                detail: None,
                description: if *self.uri != params.text_document_position.text_document.uri {
                    uri_to_file_name(&self.uri)
                } else {
                    None
                },
            }),
            data: Some(serde_json::Value::String(self.completion_data())),
            ..Default::default()
        });

        for child in &self.children {
            res.extend(child.read().to_completions(params, request_method))
        }

        res
    }

    /// Return a [Hover] from an [EnumStructItem].
    ///
    /// # Arguments
    ///
    /// * `_params` - [HoverParams] of the request.
    pub fn to_hover(&self, _params: &HoverParams) -> Option<Hover> {
        let mut contents = vec![MarkedString::LanguageString(LanguageString {
            language: "sourcepawn".to_string(),
            value: self.formatted_text(),
        })];
        if let Some(md_text) = self.description.to_md() {
            contents.push(MarkedString::String(md_text))
        }
        Some(Hover {
            contents: HoverContents::Array(contents),
            range: None,
        })
    }

    /// Return a [LocationLink] from an [EnumStructItem].
    ///
    /// # Arguments
    ///
    /// * `_params` - [GotoDefinitionParams] of the request.
    pub fn to_definition(&self, _params: &GotoDefinitionParams) -> Option<LocationLink> {
        Some(LocationLink {
            target_range: self.v_range,
            target_uri: self.uri.as_ref().clone(),
            target_selection_range: self.v_range,
            origin_selection_range: None,
        })
    }

    /// Return a [DocumentSymbol] from an [EnumStructItem].
    pub fn to_document_symbol(&self) -> Option<DocumentSymbol> {
        let mut tags = vec![];
        if self.description.deprecated.is_some() {
            tags.push(SymbolTag::DEPRECATED);
        }
        #[allow(deprecated)]
        Some(DocumentSymbol {
            name: self.name.to_string(),
            detail: None,
            kind: SymbolKind::STRUCT,
            tags: Some(tags),
            range: self.v_full_range,
            deprecated: None,
            selection_range: self.v_range,
            children: Some(
                self.children
                    .iter()
                    .filter_map(|child| child.read().to_document_symbol())
                    .collect(),
            ),
        })
    }

    /// Return a key to be used as a unique identifier in a map containing all the items.
    pub fn key(&self) -> String {
        self.name.clone()
    }

    pub fn completion_data(&self) -> String {
        format!("{}${}", self.key(), self.file_id)
    }

    /// Formatted representation of an [EnumStructItem].
    ///
    /// # Exemple
    ///
    /// `enum struct Action`
    pub(crate) fn formatted_text(&self) -> String {
        format!("enum struct {}", self.name)
    }
}
