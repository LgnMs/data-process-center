//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, TS, Default)]
#[sea_orm(table_name = "collect_log")]
#[ts(
    export,
    export_to = "ui/api/models/auto-generates/CollectLog.ts",
    rename = "CollectLog"
)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub collect_config_id: Option<i32>,
    pub task_id: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub running_log: String,
    pub status: i32,
    #[serde(skip_deserializing)]
    pub update_time: DateTime,
    #[serde(skip_deserializing)]
    pub create_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::collect_config::Entity",
        from = "Column::CollectConfigId",
        to = "super::collect_config::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    CollectConfig,
}

impl Related<super::collect_config::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CollectConfig.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
