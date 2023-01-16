//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "transfer"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq)]
pub struct Model {
    pub idx: i64,
    pub asset_transfer_idx: i64,
    pub amount: String,
    pub blinded_utxo: Option<String>,
    pub blinding_secret: Option<String>,
    pub ack: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Idx,
    AssetTransferIdx,
    Amount,
    BlindedUtxo,
    BlindingSecret,
    Ack,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Idx,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    AssetTransfer,
    TransferConsignmentEndpoint,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Idx => ColumnType::BigInteger.def(),
            Self::AssetTransferIdx => ColumnType::BigInteger.def(),
            Self::Amount => ColumnType::String(None).def(),
            Self::BlindedUtxo => ColumnType::String(None).def().null(),
            Self::BlindingSecret => ColumnType::String(None).def().null(),
            Self::Ack => ColumnType::Boolean.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::AssetTransfer => Entity::belongs_to(super::asset_transfer::Entity)
                .from(Column::AssetTransferIdx)
                .to(super::asset_transfer::Column::Idx)
                .into(),
            Self::TransferConsignmentEndpoint => {
                Entity::has_many(super::transfer_consignment_endpoint::Entity).into()
            }
        }
    }
}

impl Related<super::asset_transfer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AssetTransfer.def()
    }
}

impl Related<super::transfer_consignment_endpoint::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransferConsignmentEndpoint.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
