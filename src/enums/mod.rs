//! Enums to be used throughout the crate.
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use crate::prelude::*;

/// Possible DType returns, matches [`BlackJackData`]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, PartialOrd)]
pub enum DType {
    /// `f64`
    F64,

    /// `i64`
    I64,

    /// `f32`
    F32,

    /// `i32`
    I32,

    /// `String`
    STRING,

    /// `Decimal`
    DECIMAL,

    /// `Date`
    DATE,

    /// `Time`
    TIME,

    /// `DateTime`
    DATETIME,
}

/// Container for use with `Row` struct
#[derive(PartialEq)]
pub enum Datum<'a> {
    /// Refrence to a f64 within the dataframe
    F64(&'a f64),

    /// Refrence to a i64 within the dataframe
    I64(&'a i64),

    /// Refrence to a f32 within the dataframe
    F32(&'a f32),

    /// Refrence to a i32 within the dataframe
    I32(&'a i32),

    /// Refrence to a String within the dataframe
    STR(&'a String),

    /// Refrence to a Decimal within the dataframe
    DEC(&'a Decimal),

    /// Refrence to a Date within the dataframe
    DAT(&'a Date),

    /// Refrence to a Time within the dataframe
    TIM(&'a Time),

    /// Refrence to a DateTime within the dataframe
    DTM(&'a DateTime),
}

/// An enum representation of a `Series`, typically only seen
/// when trying to get a reference to a column/`Series` from a
/// `DataFrame` without knowing its type beforehand.
pub enum Column {
    /// A column in the `DataFrame` of type `Series<f64>`
    F64(Series<f64>),

    /// A column in the `DataFrame` of type `Series<i64>`
    I64(Series<i64>),

    /// A column in the `DataFrame` of type `Series<f32>`
    F32(Series<f32>),

    /// A column in the `DataFrame` of type `Series<i32>`
    I32(Series<i32>),

    /// A column in the `DataFrame` of type `Series<String>`
    STR(Series<String>),

    /// A column in the `DataFrame` of type `Series<Decimal>`
    DEC(Series<Decimal>),

    /// A column in the `DataFrame` of type `Series<Date>`
    DAT(Series<Date>),

    /// A column in the `DataFrame` of type `Series<Time>`
    TIM(Series<Time>),

    /// A column in the `DataFrame` of type `Series<DateTime>`
    DTM(Series<DateTime>),
}
