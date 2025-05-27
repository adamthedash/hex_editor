use anyhow::{Context, Result, bail};
use std::{collections::HashMap, iter::Peekable};

use crate::parser::{Count, DType, Endianness, Expr};

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
            DType::U16(e) => U16(chunks
                .iter()
                .map(|&x| {
                    assert_eq!(x.len(), 2);
                    match e {
                        Endianness::Big => u16::from_be_bytes(x.try_into().unwrap()),
                        Endianness::Little => u16::from_le_bytes(x.try_into().unwrap()),
                    }
                })
                .collect()),
            DType::U32(e) => U32(chunks
                .iter()
                .map(|&x| {
                    assert_eq!(x.len(), 4);
                    match e {
                        Endianness::Big => u32::from_be_bytes(x.try_into().unwrap()),
                        Endianness::Little => u32::from_le_bytes(x.try_into().unwrap()),
                    }
                })
                .collect()),
            DType::U64(e) => U64(chunks
                .iter()
                .map(|&x| {
                    assert_eq!(x.len(), 8);
                    match e {
                        Endianness::Big => u64::from_be_bytes(x.try_into().unwrap()),
                        Endianness::Little => u64::from_le_bytes(x.try_into().unwrap()),
                    }
                })
                .collect()),
            DType::U128(e) => U128(
                chunks
                    .iter()
                    .map(|&x| {
                        assert_eq!(x.len(), 16);
                        match e {
                            Endianness::Big => u128::from_be_bytes(x.try_into().unwrap()),
                            Endianness::Little => u128::from_le_bytes(x.try_into().unwrap()),
                        }
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

pub fn process_bytes(
    pattern: &[Expr],
    bytes: &mut Peekable<impl Iterator<Item = u8>>,
) -> Result<Data> {
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
                            .with_context(|| format!("Variable not found: {:?}", id))?;

                        match val {
                            PrimativeArray::U8(items) => items[0] as usize,
                            PrimativeArray::U16(items) => items[0] as usize,
                            PrimativeArray::U32(items) => items[0] as usize,
                            PrimativeArray::U64(items) => items[0] as usize,
                            PrimativeArray::U128(_) => bail!("Cannot downcast u128 -> usize"),
                            _ => bail!("Cannot use dtype as count: {:?}", val),
                        }
                    }
                };

                let bytes_per_data = match dtype {
                    DType::U8 => 1,
                    DType::U16(_) => 2,
                    DType::U32(_) => 4,
                    DType::U64(_) => 8,
                    DType::U128(_) => 16,
                    DType::Char => 1,
                };
                println!(
                    "{:?}, {:?}, {}x{}",
                    p,
                    bytes.size_hint(),
                    count,
                    bytes_per_data
                );

                let data = (0..count * bytes_per_data)
                    .map(|_| bytes.next().context("Ran out of bytes!"))
                    .collect::<Result<Vec<_>>>()
                    .with_context(|| format!("Failed to apply pattern: {:?}", p))?;
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
                    sub_parsed.push(process_bytes(exprs, bytes)?);
                }

                parsed.push(Data::List(sub_parsed));
            }
            Expr::TakeN { count, exprs } => {
                let count = match count {
                    Count::Number(n) => *n as usize,
                    Count::Identifier(id) => {
                        let val = variables
                            .get(id)
                            .with_context(|| format!("Variable not found: {:?}", id))?;

                        match val {
                            PrimativeArray::U8(items) => items[0] as usize,
                            PrimativeArray::U16(items) => items[0] as usize,
                            PrimativeArray::U32(items) => items[0] as usize,
                            PrimativeArray::U64(items) => items[0] as usize,
                            PrimativeArray::U128(_) => bail!("Cannot downcast u128 -> usize"),
                            _ => bail!("Cannot use dtype as count: {:?}", val),
                        }
                    }
                };

                let mut sub_parsed = vec![];
                for i in 0..count {
                    sub_parsed.push(
                        process_bytes(exprs, bytes)
                            .with_context(|| format!("Failed to parse TAKE_N item #{}", i))?,
                    );
                }

                parsed.push(Data::List(sub_parsed));
            }
        }
    }

    Ok(Data::List(parsed))
}
