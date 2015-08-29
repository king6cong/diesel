extern crate postgres;

use {Queriable, QuerySource};

use self::postgres::rows::Row;

pub struct Serial;
pub struct VarChar;

pub trait NativeSqlType {}

pub trait FromSql<A: NativeSqlType> {
    fn from_sql(row: &Row, idx: usize) -> Self;
}

macro_rules! primitive_impls {
    ($($Source:ident -> $Target:ident),+,) => {
        $(
            impl NativeSqlType for $Source {}
            impl FromSql<$Source> for $Target {
                fn from_sql(row: &Row, idx: usize) -> Self {
                    row.get(idx)
                }
            }

            impl<QS> Queriable<QS> for $Target where
                QS: QuerySource<SqlType=$Source>
            {
                type Row = Self;

                fn build(row: Self::Row) -> Self {
                    row
                }
            }
        )+
    }
}

primitive_impls! {
    Serial -> i32,
    VarChar -> String,
}

macro_rules! tuple_impls {
    ($(
        $Tuple:ident {
            $(($idx:expr) -> $T:ident, $ST:ident,)+
        }
    )+) => {
        $(
            impl<$($T:NativeSqlType),+> NativeSqlType for ($($T,)+) {}
            impl<$($T),+,$($ST),+> FromSql<($($ST),+)> for ($($T),+) where
                $($T: FromSql<$ST>),+,
                $($ST: NativeSqlType),+
            {
                fn from_sql(row: &Row, idx: usize) -> Self {
                    (
                        $($T::from_sql(row, idx + $idx)),+
                    )
                }
            }
        )+
    }
}

tuple_impls! {
    T2 {
        (0) -> A, SA,
        (1) -> B, SB,
    }
    T3 {
        (0) -> A, SA,
        (1) -> B, SB,
        (2) -> C, SC,
    }
    T4 {
        (0) -> A, SA,
        (1) -> B, SB,
        (2) -> C, SC,
        (3) -> D, SD,
    }
    T5 {
        (0) -> A, SA,
        (1) -> B, SB,
        (2) -> C, SC,
        (3) -> D, SD,
        (4) -> E, SE,
    }
    T6 {
        (0) -> A, SA,
        (1) -> B, SB,
        (2) -> C, SC,
        (3) -> D, SD,
        (4) -> E, SE,
        (5) -> F, SF,
    }
    T7 {
        (0) -> A, SA,
        (1) -> B, SB,
        (2) -> C, SC,
        (3) -> D, SD,
        (4) -> E, SE,
        (5) -> F, SF,
        (6) -> G, SG,
    }
    T8 {
        (0) -> A, SA,
        (1) -> B, SB,
        (2) -> C, SC,
        (3) -> D, SD,
        (4) -> E, SE,
        (5) -> F, SF,
        (6) -> G, SG,
        (7) -> H, SH,
    }
    T9 {
        (0) -> A, SA,
        (1) -> B, SB,
        (2) -> C, SC,
        (3) -> D, SD,
        (4) -> E, SE,
        (5) -> F, SF,
        (6) -> G, SG,
        (7) -> H, SH,
        (8) -> I, SI,
    }
    T10 {
        (0) -> A, SA,
        (1) -> B, SB,
        (2) -> C, SC,
        (3) -> D, SD,
        (4) -> E, SE,
        (5) -> F, SF,
        (6) -> G, SG,
        (7) -> H, SH,
        (8) -> I, SI,
        (9) -> J, SJ,
    }
    T11 {
        (0) -> A, SA,
        (1) -> B, SB,
        (2) -> C, SC,
        (3) -> D, SD,
        (4) -> E, SE,
        (5) -> F, SF,
        (6) -> G, SG,
        (7) -> H, SH,
        (8) -> I, SI,
        (9) -> J, SJ,
        (10) -> K, SK,
    }
    T12 {
        (0) -> A, SA,
        (1) -> B, SB,
        (2) -> C, SC,
        (3) -> D, SD,
        (4) -> E, SE,
        (5) -> F, SF,
        (6) -> G, SG,
        (7) -> H, SH,
        (8) -> I, SI,
        (9) -> J, SJ,
        (10) -> K, SK,
        (11) -> L, SL,
    }
}
