use std::sync::Arc;
use strum_macros::Display;

use flowy_derive::{Flowy_Event, ProtoBuf_Enum};
use lib_dispatch::prelude::AFPlugin;

use crate::event_handler::get_snapshot_handler;
use crate::{event_handler::*, manager::DocumentManager};

pub fn init(document_manager: Arc<DocumentManager>) -> AFPlugin {
  AFPlugin::new()
    .name(env!("CARGO_PKG_NAME"))
    .state(document_manager)
    .event(DocumentEvent::CreateDocument, create_document_handler)
    .event(DocumentEvent::OpenDocument, open_document_handler)
    .event(DocumentEvent::CloseDocument, close_document_handler)
    .event(DocumentEvent::ApplyAction, apply_action_handler)
    .event(DocumentEvent::GetDocumentData, get_document_data_handler)
    .event(
      DocumentEvent::ConvertDataToDocument,
      convert_data_to_document,
    )
    .event(DocumentEvent::Redo, redo_handler)
    .event(DocumentEvent::Undo, undo_handler)
    .event(DocumentEvent::CanUndoRedo, can_undo_redo_handler)
    .event(DocumentEvent::GetDocumentSnapshots, get_snapshot_handler)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, ProtoBuf_Enum, Flowy_Event)]
#[event_err = "FlowyError"]
pub enum DocumentEvent {
  #[event(input = "CreateDocumentPayloadPB")]
  CreateDocument = 0,

  #[event(input = "OpenDocumentPayloadPB", output = "DocumentDataPB")]
  OpenDocument = 1,

  #[event(input = "CloseDocumentPayloadPB")]
  CloseDocument = 2,

  #[event(input = "ApplyActionPayloadPB")]
  ApplyAction = 3,

  #[event(input = "OpenDocumentPayloadPB", output = "DocumentDataPB")]
  GetDocumentData = 4,

  #[event(input = "ConvertDataPayloadPB", output = "DocumentDataPB")]
  ConvertDataToDocument = 5,

  #[event(
    input = "DocumentRedoUndoPayloadPB",
    output = "DocumentRedoUndoResponsePB"
  )]
  Redo = 6,

  #[event(
    input = "DocumentRedoUndoPayloadPB",
    output = "DocumentRedoUndoResponsePB"
  )]
  Undo = 7,

  #[event(
    input = "DocumentRedoUndoPayloadPB",
    output = "DocumentRedoUndoResponsePB"
  )]
  CanUndoRedo = 8,

  #[event(input = "OpenDocumentPayloadPB", output = "RepeatedDocumentSnapshotPB")]
  GetDocumentSnapshots = 9,
}
