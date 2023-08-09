use lsp_types::{SignatureHelp, SignatureHelpParams, SignatureInformation};

use crate::store::Store;

use self::signature_attributes::SignatureAttributes;

mod signature_attributes;

pub fn provide_signature_help(store: &Store, params: SignatureHelpParams) -> Option<SignatureHelp> {
    let uri = params.text_document_position_params.text_document.uri;
    let document = store.documents.get(&uri)?;
    let signature_attributes = SignatureAttributes::get_signature_attributes(
        document,
        params.text_document_position_params.position,
    )?;

    let items = &store.get_items_from_position(signature_attributes.position, &uri);
    let mut signatures: Vec<SignatureInformation> = Vec::new();
    for item in items {
        let signature_help = item
            .read()
            .unwrap()
            .to_signature_help(signature_attributes.parameter_count);
        if let Some(signature_help) = signature_help {
            signatures.push(signature_help);
        }
    }

    Some(SignatureHelp {
        signatures,
        active_parameter: None,
        active_signature: None,
    })
}
