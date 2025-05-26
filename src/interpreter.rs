use std::{collections::HashMap, iter::Peekable};

use crate::parser::{Count, DType, Expr};

#[derive(Debug, Clone)]
pub enum PrimativeArray {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    U64(Vec<u64>),
    U128(Vec<u128>),
    Char(Vec<u8>),
}

#[derive(Debug, Clone)]
pub enum Data {
    Primative(PrimativeArray),
    List(Vec<Data>),
}

impl PrimativeArray {
    fn from_chunked_array(chunks: &[&[u8]], dtype: &DType) -> Self {
        use PrimativeArray::*;
        match dtype {
            DType::U8 => U8(chunks
                .iter()
                .map(|x| {
                    assert_eq!(x.len(), 1);
                    x[0]
                })
                .collect()),
            DType::U16 => U16(chunks
                .iter()
                .map(|&x| {
                    assert_eq!(x.len(), 2);
                    u16::from_be_bytes(x.try_into().unwrap())
                })
                .collect()),
            DType::U32 => U32(chunks
                .iter()
                .map(|&x| {
                    assert_eq!(x.len(), 4);
                    u32::from_be_bytes(x.try_into().unwrap())
                })
                .collect()),
            DType::U64 => U64(chunks
                .iter()
                .map(|&x| {
                    assert_eq!(x.len(), 8);
                    u64::from_be_bytes(x.try_into().unwrap())
                })
                .collect()),
            DType::U128 => U128(
                chunks
                    .iter()
                    .map(|&x| {
                        assert_eq!(x.len(), 16);
                        u128::from_be_bytes(x.try_into().unwrap())
                    })
                    .collect(),
            ),
            DType::Char => Char(
                chunks
                    .iter()
                    .map(|x| {
                        assert_eq!(x.len(), 1);
                        x[0]
                    })
                    .collect(),
            ),
        }
    }
}

pub fn process_bytes(pattern: &[Expr], bytes: &mut Peekable<impl Iterator<Item = u8>>) -> Data {
    let mut variables = HashMap::<String, PrimativeArray>::new();

    let mut parsed = vec![];
    for p in pattern {
        match p {
            Expr::Primative {
                dtype,
                count,
                identifier,
            } => {
                let count = match count {
                    Count::Number(n) => *n as usize,
                    Count::Identifier(id) => {
                        let val = variables
                            .get(id)
                            .unwrap_or_else(|| panic!("Variable not found: {}", id));

                        match val {
                            PrimativeArray::U8(items) => items[0] as usize,
                            PrimativeArray::U16(items) => items[0] as usize,
                            PrimativeArray::U32(items) => items[0] as usize,
                            PrimativeArray::U64(items) => items[0] as usize,
                            PrimativeArray::U128(items) => panic!("Cannot downcast u128 -> usize"),
                            _ => panic!("Cannot use dtype as count: {:?}", val),
                        }
                    }
                };

                let bytes_per_data = match dtype {
                    DType::U8 => 1,
                    DType::U16 => 2,
                    DType::U32 => 4,
                    DType::U64 => 8,
                    DType::U128 => 16,
                    DType::Char => 1,
                };
                // println!(
                //     "{:?}, {:?}, {}x{}",
                //     p,
                //     bytes.size_hint(),
                //     count,
                //     bytes_per_data
                // );

                let data = (0..count * bytes_per_data)
                    .map(|_| bytes.next().expect("Ran out of bytes!"))
                    .collect::<Vec<_>>();
                let data = data.chunks_exact(bytes_per_data).collect::<Vec<_>>();
                let primative = PrimativeArray::from_chunked_array(&data, dtype);

                if let Some(id) = identifier {
                    variables.entry(id.clone()).insert_entry(primative.clone());
                };

                parsed.push(Data::Primative(primative));
            }
            Expr::TakeUntil(exprs) => {
                let mut sub_parsed = vec![];
                while bytes.peek().is_some() {
                    sub_parsed.push(process_bytes(exprs, bytes));
                }

                parsed.push(Data::List(sub_parsed));
            }
        }
    }

    Data::List(parsed)
}
