use std::io::Write;

use colored::Colorize;
use rand::random;

use crate::interpreter::{Data, PrimativeArray};

pub fn print_data(data: &Data, stack_colors: &[(u8, u8, u8)]) {
    match data {
        Data::Primative(primative_array) => {
            let stack_prefix = stack_colors
                .iter()
                .map(|&(r, g, b)| "  ".on_truecolor(r, g, b));

            for s in stack_prefix {
                print!("{}", s);
            }

            let string = match primative_array {
                PrimativeArray::U8(items) => items
                    .iter()
                    .map(|x| format!("{x:0>2x} "))
                    .collect::<Vec<_>>(),
                PrimativeArray::U16(items) => {
                    items.iter().map(|x| format!("{x} ")).collect::<Vec<_>>()
                }
                PrimativeArray::U32(items) => {
                    items.iter().map(|x| format!("{x} ")).collect::<Vec<_>>()
                }
                PrimativeArray::U64(items) => {
                    items.iter().map(|x| format!("{x} ")).collect::<Vec<_>>()
                }
                PrimativeArray::U128(items) => {
                    items.iter().map(|x| format!("{x} ")).collect::<Vec<_>>()
                }
                PrimativeArray::Char(items) => items
                    .iter()
                    .map(|x| format!("{} ", std::ascii::escape_default(*x)))
                    .collect::<Vec<_>>(),
            };
            for s in string {
                print!("{}", s);
            }
            println!();
            std::io::stdout().flush().unwrap();
        }
        Data::List(datas) => {
            let mut stack_colors = stack_colors.to_vec();
            stack_colors.push((random(), random(), random()));
            datas
                .iter()
                .for_each(|data| print_data(data, &stack_colors));
        }
    }
}
