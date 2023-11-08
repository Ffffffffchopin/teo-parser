use std::fmt::{Display, Formatter};
use educe::Educe;
use serde::Serialize;
use crate::r#type::Type;

#[derive(Debug, Clone, Eq, Serialize)]
#[derive(Educe)]
#[educe(Hash, PartialEq)]
pub enum SynthesizedShape {
    BoolFilter,
    BoolNullableFilter,
    IntFilter,
    IntNullableFilter,
    Int64Filter,
    Int64NullableFilter,
    Float32Filter,
    Float32NullableFilter,
    FloatFilter,
    FloatNullableFilter,
    DecimalFilter,
    DecimalNullableFilter,
    DateFilter,
    DateNullableFilter,
    DateTimeFilter,
    DateTimeNullableFilter,
    ObjectIdFilter,
    ObjectIdNullableFilter,
    StringFilter,
    StringNullableFilter,
    EnumFilter(Box<Type>),
    EnumNullableFilter(Box<Type>),
    ArrayFilter(Box<Type>),
    ArrayNullableFilter(Box<Type>),
    BoolWithAggregatesFilter,
    BoolNullableWithAggregatesFilter,
    IntWithAggregatesFilter,
    IntNullableWithAggregatesFilter,
    Int64WithAggregatesFilter,
    Int64NullableWithAggregatesFilter,
    Float32WithAggregatesFilter,
    Float32NullableWithAggregatesFilter,
    FloatWithAggregatesFilter,
    FloatNullableWithAggregatesFilter,
    DecimalWithAggregatesFilter,
    DecimalNullableWithAggregatesFilter,
    DateWithAggregatesFilter,
    DateNullableWithAggregatesFilter,
    DateTimeWithAggregatesFilter,
    DateTimeNullableWithAggregatesFilter,
    ObjectIdWithAggregatesFilter,
    ObjectIdNullableWithAggregatesFilter,
    StringWithAggregatesFilter,
    StringNullableWithAggregatesFilter,
    EnumWithAggregatesFilter(Box<Type>),
    EnumNullableWithAggregatesFilter(Box<Type>),
    ArrayWithAggregatesFilter(Box<Type>),
    ArrayNullableWithAggregatesFilter(Box<Type>),
    IntAtomicUpdateOperationInput,
    Int64AtomicUpdateOperationInput,
    Float32AtomicUpdateOperationInput,
    FloatAtomicUpdateOperationInput,
    DecimalAtomicUpdateOperationInput,
    ArrayAtomicUpdateOperationInput(Box<Type>),
    Args(Vec<usize>, Vec<String>),
    FindManyArgs(Vec<usize>, Vec<String>),
    FindFirstArgs(Vec<usize>, Vec<String>),
    FindUniqueArgs(Vec<usize>, Vec<String>),
    CreateArgs(Vec<usize>, Vec<String>),
    UpdateArgs(Vec<usize>, Vec<String>),
    UpsertArgs(Vec<usize>, Vec<String>),
    CopyArgs(Vec<usize>, Vec<String>),
    DeleteArgs(Vec<usize>, Vec<String>),
    CreateManyArgs(Vec<usize>, Vec<String>),
    UpdateManyArgs(Vec<usize>, Vec<String>),
    CopyManyArgs(Vec<usize>, Vec<String>),
    DeleteManyArgs(Vec<usize>, Vec<String>),
    CountArgs(Vec<usize>, Vec<String>),
    AggregateArgs(Vec<usize>, Vec<String>),
    GroupByArgs(Vec<usize>, Vec<String>),
    RelationFilter(Vec<usize>, Vec<String>),
    ListRelationFilter(Vec<usize>, Vec<String>),
    WhereInput(Vec<usize>, Vec<String>),
    WhereUniqueInput(Vec<usize>, Vec<String>),
    ScalarFieldEnum(Vec<usize>, Vec<String>),
    ScalarWhereWithAggregatesInput(Vec<usize>, Vec<String>),
    CountAggregateInputType(Vec<usize>, Vec<String>),
    SumAggregateInputType(Vec<usize>, Vec<String>),
    AvgAggregateInputType(Vec<usize>, Vec<String>),
    MaxAggregateInputType(Vec<usize>, Vec<String>),
    MinAggregateInputType(Vec<usize>, Vec<String>),
    CreateInput(Vec<usize>, Vec<String>),
    CreateInputWithout(Vec<usize>, Vec<String>, String),
    CreateNestedOneInput(Vec<usize>, Vec<String>),
    CreateNestedOneInputWithout(Vec<usize>, Vec<String>, String),
    CreateNestedManyInput(Vec<usize>, Vec<String>),
    CreateNestedManyInputWithout(Vec<usize>, Vec<String>, String),
    UpdateInput(Vec<usize>, Vec<String>),
    UpdateInputWithout(Vec<usize>, Vec<String>, String),
    UpdateNestedOneInput(Vec<usize>, Vec<String>),
    UpdateNestedOneInputWithout(Vec<usize>, Vec<String>, String),
    UpdateNestedManyInput(Vec<usize>, Vec<String>),
    UpdateNestedManyInputWithout(Vec<usize>, Vec<String>, String),
    ConnectOrCreateInput(Vec<usize>, Vec<String>),
    ConnectOrCreateInputWithout(Vec<usize>, Vec<String>, String),
    UpdateWithWhereUniqueInput(Vec<usize>, Vec<String>),
    UpdateWithWhereUniqueInputWithout(Vec<usize>, Vec<String>, String),
    UpsertWithWhereUniqueInput(Vec<usize>, Vec<String>),
    UpsertWithWhereUniqueInputWithout(Vec<usize>, Vec<String>, String),
    UpdateManyWithWhereInput(Vec<usize>, Vec<String>),
    UpdateManyWithWhereInputWithout(Vec<usize>, Vec<String>, String),
    Select(Vec<usize>, Vec<String>),
    Include(Vec<usize>, Vec<String>),
    OrderByInput(Vec<usize>, Vec<String>),
    Result(Vec<usize>, Vec<String>),
    CountAggregateResult(Vec<usize>, Vec<String>),
    SumAggregateResult(Vec<usize>, Vec<String>),
    AvgAggregateResult(Vec<usize>, Vec<String>),
    MinAggregateResult(Vec<usize>, Vec<String>),
    MaxAggregateResult(Vec<usize>, Vec<String>),
    AggregateResult(Vec<usize>, Vec<String>),
    GroupByResult(Vec<usize>, Vec<String>),
}

impl Display for SynthesizedShape {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SynthesizedShape::BoolFilter => f.write_str("BoolFilter"),
            SynthesizedShape::BoolNullableFilter => f.write_str("BoolNullableFilter"),
            SynthesizedShape::IntFilter => f.write_str("IntFilter"),
            SynthesizedShape::IntNullableFilter => f.write_str("IntNullableFilter"),
            SynthesizedShape::Int64Filter => f.write_str("Int64Filter"),
            SynthesizedShape::Int64NullableFilter => f.write_str("Int64NullableFilter"),
            SynthesizedShape::Float32Filter => f.write_str("Float32Filter"),
            SynthesizedShape::Float32NullableFilter => f.write_str("Float32NullableFilter"),
            SynthesizedShape::FloatFilter => f.write_str("FloatFilter"),
            SynthesizedShape::FloatNullableFilter => f.write_str("FloatNullableFilter"),
            SynthesizedShape::DecimalFilter => f.write_str("DecimalFilter"),
            SynthesizedShape::DecimalNullableFilter => f.write_str("DecimalNullableFilter"),
            SynthesizedShape::DateFilter => f.write_str("DateFilter"),
            SynthesizedShape::DateNullableFilter => f.write_str("DateNullableFilter"),
            SynthesizedShape::DateTimeFilter => f.write_str("DateTimeFilter"),
            SynthesizedShape::DateTimeNullableFilter => f.write_str("DateTimeNullableFilter"),
            SynthesizedShape::ObjectIdFilter => f.write_str("ObjectIdFilter"),
            SynthesizedShape::ObjectIdNullableFilter => f.write_str("ObjectIdNullableFilter"),
            SynthesizedShape::StringFilter => f.write_str("StringFilter"),
            SynthesizedShape::StringNullableFilter => f.write_str("StringNullableFilter"),
            SynthesizedShape::EnumFilter(t) => f.write_str(&format!("EnumFilter<{}>", t.as_ref())),
            SynthesizedShape::EnumNullableFilter(t) => f.write_str(&format!("EnumNullableFilter<{}>", t.as_ref())),
            SynthesizedShape::ArrayFilter(t) => f.write_str(&format!("ArrayFilter<{}>", t.as_ref())),
            SynthesizedShape::ArrayNullableFilter(t) => f.write_str(&format!("ArrayNullableFilter<{}>", t.as_ref())),
            SynthesizedShape::BoolWithAggregatesFilter => f.write_str("BoolWithAggregatesFilter"),
            SynthesizedShape::BoolNullableWithAggregatesFilter => f.write_str("BoolNullableWithAggregatesFilter"),
            SynthesizedShape::IntWithAggregatesFilter => f.write_str("IntWithAggregatesFilter"),
            SynthesizedShape::IntNullableWithAggregatesFilter => f.write_str("IntNullableWithAggregatesFilter"),
            SynthesizedShape::Int64WithAggregatesFilter => f.write_str("Int64WithAggregatesFilter"),
            SynthesizedShape::Int64NullableWithAggregatesFilter => f.write_str("Int64NullableWithAggregatesFilter"),
            SynthesizedShape::Float32WithAggregatesFilter => f.write_str("Float32WithAggregatesFilter"),
            SynthesizedShape::Float32NullableWithAggregatesFilter => f.write_str("Float32NullableWithAggregatesFilter"),
            SynthesizedShape::FloatWithAggregatesFilter => f.write_str("FloatWithAggregatesFilter"),
            SynthesizedShape::FloatNullableWithAggregatesFilter => f.write_str("FloatNullableWithAggregatesFilter"),
            SynthesizedShape::DecimalWithAggregatesFilter => f.write_str("DecimalWithAggregatesFilter"),
            SynthesizedShape::DecimalNullableWithAggregatesFilter => f.write_str("DecimalNullableWithAggregatesFilter"),
            SynthesizedShape::DateWithAggregatesFilter => f.write_str("DateWithAggregatesFilter"),
            SynthesizedShape::DateNullableWithAggregatesFilter => f.write_str("DateNullableWithAggregatesFilter"),
            SynthesizedShape::DateTimeWithAggregatesFilter => f.write_str("DateTimeWithAggregatesFilter"),
            SynthesizedShape::DateTimeNullableWithAggregatesFilter => f.write_str("DateTimeNullableWithAggregatesFilter"),
            SynthesizedShape::ObjectIdWithAggregatesFilter => f.write_str("ObjectIdWithAggregatesFilter"),
            SynthesizedShape::ObjectIdNullableWithAggregatesFilter => f.write_str("ObjectIdNullableWithAggregatesFilter"),
            SynthesizedShape::StringWithAggregatesFilter => f.write_str("StringWithAggregatesFilter"),
            SynthesizedShape::StringNullableWithAggregatesFilter => f.write_str("StringNullableWithAggregatesFilter"),
            SynthesizedShape::EnumWithAggregatesFilter(t) => f.write_str(&format!("EnumWithAggregatesFilter<{}>", t.as_ref())),
            SynthesizedShape::EnumNullableWithAggregatesFilter(t) => f.write_str(&format!("EnumNullableWithAggregatesFilter<{}>", t.as_ref())),
            SynthesizedShape::ArrayWithAggregatesFilter(t) => f.write_str(&format!("ArrayWithAggregatesFilter<{}>", t.as_ref())),
            SynthesizedShape::ArrayNullableWithAggregatesFilter(t) => f.write_str(&format!("ArrayNullableWithAggregatesFilter<{}>", t.as_ref())),
            SynthesizedShape::IntAtomicUpdateOperationInput => f.write_str("IntAtomicUpdateOperationInput"),
            SynthesizedShape::Int64AtomicUpdateOperationInput => f.write_str("Int64AtomicUpdateOperationInput"),
            SynthesizedShape::Float32AtomicUpdateOperationInput => f.write_str("Float32AtomicUpdateOperationInput"),
            SynthesizedShape::FloatAtomicUpdateOperationInput => f.write_str("FloatAtomicUpdateOperationInput"),
            SynthesizedShape::DecimalAtomicUpdateOperationInput => f.write_str("DecimalAtomicUpdateOperationInput"),
            SynthesizedShape::ArrayAtomicUpdateOperationInput(t) => f.write_str(&format!("ArrayAtomicUpdateOperationInput{}", t.as_ref())),
            SynthesizedShape::Args(_, k) => f.write_str(&format!("Args<{}>", k.join("."))),
            SynthesizedShape::FindManyArgs(_, k) => f.write_str(&format!("FindManyArgs<{}>", k.join("."))),
            SynthesizedShape::FindFirstArgs(_, k) => f.write_str(&format!("FindFirstArgs<{}>", k.join("."))),
            SynthesizedShape::FindUniqueArgs(_, k) => f.write_str(&format!("FindUniqueArgs<{}>", k.join("."))),
            SynthesizedShape::CreateArgs(_, k) => f.write_str(&format!("CreateArgs<{}>", k.join("."))),
            SynthesizedShape::UpdateArgs(_, k) => f.write_str(&format!("UpdateArgs<{}>", k.join("."))),
            SynthesizedShape::UpsertArgs(_, k) => f.write_str(&format!("UpsertArgs<{}>", k.join("."))),
            SynthesizedShape::CopyArgs(_, k) => f.write_str(&format!("CopyArgs<{}>", k.join("."))),
            SynthesizedShape::DeleteArgs(_, k) => f.write_str(&format!("DeleteArgs<{}>", k.join("."))),
            SynthesizedShape::CreateManyArgs(_, k) => f.write_str(&format!("CreateManyArgs<{}>", k.join("."))),
            SynthesizedShape::UpdateManyArgs(_, k) => f.write_str(&format!("UpdateManyArgs<{}>", k.join("."))),
            SynthesizedShape::CopyManyArgs(_, k) => f.write_str(&format!("CopyManyArgs<{}>", k.join("."))),
            SynthesizedShape::DeleteManyArgs(_, k) => f.write_str(&format!("DeleteManyArgs<{}>", k.join("."))),
            SynthesizedShape::CountArgs(_, k) => f.write_str(&format!("CountArgs<{}>", k.join("."))),
            SynthesizedShape::AggregateArgs(_, k) => f.write_str(&format!("AggregateArgs<{}>", k.join("."))),
            SynthesizedShape::GroupByArgs(_, k) => f.write_str(&format!("GroupByArgs<{}>", k.join("."))),
            SynthesizedShape::RelationFilter(_, k) => f.write_str(&format!("RelationFilter<{}>", k.join("."))),
            SynthesizedShape::ListRelationFilter(_, k) => f.write_str(&format!("ListRelationFilter<{}>", k.join("."))),
            SynthesizedShape::WhereInput(_, k) => f.write_str(&format!("WhereInput<{}>", k.join("."))),
            SynthesizedShape::WhereUniqueInput(_, k) => f.write_str(&format!("WhereUniqueInput<{}>", k.join("."))),
            SynthesizedShape::ScalarFieldEnum(_, k) => f.write_str(&format!("ScalarFieldEnum<{}>", k.join("."))),
            SynthesizedShape::ScalarWhereWithAggregatesInput(_, k) => f.write_str(&format!("ScalarWhereWithAggregatesInput<{}>", k.join("."))),
            SynthesizedShape::CountAggregateInputType(_, k) => f.write_str(&format!("CountAggregateInputType<{}>", k.join("."))),
            SynthesizedShape::SumAggregateInputType(_, k) => f.write_str(&format!("SumAggregateInputType<{}>", k.join("."))),
            SynthesizedShape::AvgAggregateInputType(_, k) => f.write_str(&format!("AvgAggregateInputType<{}>", k.join("."))),
            SynthesizedShape::MaxAggregateInputType(_, k) => f.write_str(&format!("MaxAggregateInputType<{}>", k.join("."))),
            SynthesizedShape::MinAggregateInputType(_, k) => f.write_str(&format!("MinAggregateInputType<{}>", k.join("."))),
            SynthesizedShape::CreateInput(_, k) => f.write_str(&format!("CreateInput<{}>", k.join("."))),
            SynthesizedShape::CreateInputWithout(_, k, r) => f.write_str(&format!("CreateInputWithout<{}, .{}>", k.join("."), r)),
            SynthesizedShape::CreateNestedOneInput(_, k) => f.write_str(&format!("CreateNestedOneInput<{}>", k.join("."))),
            SynthesizedShape::CreateNestedOneInputWithout(_, k, r) => f.write_str(&format!("CreateNestedOneInputWithout<{}, .{}>", k.join("."), r)),
            SynthesizedShape::CreateNestedManyInput(_, k) => f.write_str(&format!("CreateNestedManyInput<{}>", k.join("."))),
            SynthesizedShape::CreateNestedManyInputWithout(_, k, r) => f.write_str(&format!("CreateNestedManyInputWithout<{}, .{}>", k.join("."), r)),
            SynthesizedShape::UpdateInput(_, k) => f.write_str(&format!("UpdateInput<{}>", k.join("."))),
            SynthesizedShape::UpdateInputWithout(_, k, r) => f.write_str(&format!("UpdateInputWithout<{}, .{}>", k.join("."), r)),
            SynthesizedShape::UpdateNestedOneInput(_, k) => f.write_str(&format!("UpdateNestedOneInput<{}>", k.join("."))),
            SynthesizedShape::UpdateNestedOneInputWithout(_, k, r) => f.write_str(&format!("UpdateNestedOneInputWithout<{}, .{}>", k.join("."), r)),
            SynthesizedShape::UpdateNestedManyInput(_, k) => f.write_str(&format!("UpdateNestedManyInput<{}>", k.join("."))),
            SynthesizedShape::UpdateNestedManyInputWithout(_, k, r) => f.write_str(&format!("UpdateNestedManyInputWithout<{}, .{}>", k.join("."), r)),
            SynthesizedShape::ConnectOrCreateInput(_, k) => f.write_str(&format!("ConnectOrCreateInput<{}>", k.join("."))),
            SynthesizedShape::ConnectOrCreateInputWithout(_, k, r) => f.write_str(&format!("ConnectOrCreateInputWithout<{}, .{}>", k.join("."), r)),
            SynthesizedShape::UpdateWithWhereUniqueInput(_, k) => f.write_str(&format!("UpdateWithWhereUniqueInput<{}>", k.join("."))),
            SynthesizedShape::UpdateWithWhereUniqueInputWithout(_, k, r) => f.write_str(&format!("UpdateWithWhereUniqueInput<{}, .{}>", k.join("."), r)),
            SynthesizedShape::UpsertWithWhereUniqueInput(_, k) => f.write_str(&format!("UpsertWithWhereUniqueInput<{}>", k.join("."))),
            SynthesizedShape::UpsertWithWhereUniqueInputWithout(_, k, r) => f.write_str(&format!("UpsertWithWhereUniqueInput<{}, .{}>", k.join("."), r)),
            SynthesizedShape::UpdateManyWithWhereInput(_, k) => f.write_str(&format!("UpdateManyWithWhereInput<{}>", k.join("."))),
            SynthesizedShape::UpdateManyWithWhereInputWithout(_, k, r) => f.write_str(&format!("UpdateManyWithWhereInput<{}, .{}>", k.join("."), r)),
            SynthesizedShape::Select(_, k) => f.write_str(&format!("Select<{}>", k.join("."))),
            SynthesizedShape::Include(_, k) => f.write_str(&format!("Include<{}>", k.join("."))),
            SynthesizedShape::OrderByInput(_, k) => f.write_str(&format!("OrderByInput<{}>", k.join("."))),
            SynthesizedShape::Result(_, k) => f.write_str(&format!("Result<{}>", k.join("."))),
            SynthesizedShape::CountAggregateResult(_, k) => f.write_str(&format!("CountAggregateResult<{}>", k.join("."))),
            SynthesizedShape::SumAggregateResult(_, k) => f.write_str(&format!("SumAggregateResult<{}>", k.join("."))),
            SynthesizedShape::AvgAggregateResult(_, k) => f.write_str(&format!("AvgAggregateResult<{}>", k.join("."))),
            SynthesizedShape::MinAggregateResult(_, k) => f.write_str(&format!("MinAggregateResult<{}>", k.join("."))),
            SynthesizedShape::MaxAggregateResult(_, k) => f.write_str(&format!("MaxAggregateResult<{}>", k.join("."))),
            SynthesizedShape::AggregateResult(_, k) => f.write_str(&format!("AggregateResult<{}>", k.join("."))),
            SynthesizedShape::GroupByResult(_, k) => f.write_str(&format!("GroupByResult<{}>", k.join("."))),
        }
    }
}