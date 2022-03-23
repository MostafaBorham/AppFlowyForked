use crate::event_handler::*;
use crate::manager::GridManager;
use flowy_derive::{Flowy_Event, ProtoBuf_Enum};
use lib_dispatch::prelude::*;
use std::sync::Arc;
use strum_macros::Display;

pub fn create(grid_manager: Arc<GridManager>) -> Module {
    let mut module = Module::new().name(env!("CARGO_PKG_NAME")).data(grid_manager);
    module = module
        .event(GridEvent::GetGridData, get_grid_data_handler)
        .event(GridEvent::GetGridBlocks, get_grid_blocks_handler)
        .event(GridEvent::GetFields, get_fields_handler)
        .event(GridEvent::UpdateField, update_field_handler)
        .event(GridEvent::CreateField, create_field_handler)
        .event(GridEvent::CreateRow, create_row_handler)
        .event(GridEvent::GetRow, get_row_handler)
        .event(GridEvent::UpdateCell, update_cell_handler);

    module
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display, Hash, ProtoBuf_Enum, Flowy_Event)]
#[event_err = "FlowyError"]
pub enum GridEvent {
    #[event(input = "GridId", output = "Grid")]
    GetGridData = 0,

    #[event(input = "QueryGridBlocksPayload", output = "RepeatedGridBlock")]
    GetGridBlocks = 1,

    #[event(input = "QueryFieldPayload", output = "RepeatedField")]
    GetFields = 10,

    #[event(input = "FieldChangeset")]
    UpdateField = 11,

    #[event(input = "CreateFieldPayload")]
    CreateField = 12,

    #[event(input = "CreateRowPayload", output = "Row")]
    CreateRow = 21,

    #[event(input = "QueryRowPayload", output = "Row")]
    GetRow = 22,

    #[event(input = "CellMetaChangeset")]
    UpdateCell = 30,
}
