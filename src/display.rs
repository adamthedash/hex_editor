use std::fmt::Write as _;
use std::io::Write;

use colored::{ColoredString, Colorize};
use rand::random;

use crate::interpreter::{Data, PrimativeArray};

pub fn print_vertical(data: &Data, stack_colors: &[(u8, u8, u8)]) {
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
            if string.len() > 32 {
                for s in &string[..16] {
                    print!("{}", s);
                }
                print!("... ");
                for s in &string[string.len() - 16..] {
                    print!("{}", s);
                }

                print!("(len = {})", string.len());
            } else {
                for s in string {
                    print!("{}", s);
                }
            }
            println!();
            std::io::stdout().flush().unwrap();
        }
        Data::List(datas) => {
            let mut stack_colors = stack_colors.to_vec();
            stack_colors.push((random(), random(), random()));
            datas
                .iter()
                .for_each(|data| print_vertical(data, &stack_colors));
        }
    }
}

/// Calculates whether white/black should be used for foreground text
fn get_contrasting_color(color: (u8, u8, u8)) -> (u8, u8, u8) {
    let luminance =
        (0.299 * color.0 as f32 + 0.587 * color.1 as f32 + 0.114 * color.2 as f32) / 255.;

    if luminance > 0.5 {
        (0, 0, 0)
    } else {
        (255, 255, 255)
    }
}

/// Handles horizontal printing of several rows in sync
pub struct HexWriter {
    hex_buffer: String,
    screen_width: usize,
    color_buffers: Vec<Vec<ColoredString>>,
}

impl HexWriter {
    pub fn new(screen_width: usize) -> Self {
        Self {
            hex_buffer: String::new(),
            screen_width,
            color_buffers: vec![],
        }
    }

    fn print(&mut self) {
        //eprintln!("hex len: {}", self.hex_buffer.len());
        //eprintln!(
        //    "dec len: {}",
        //    self.dec_buffer
        //        .iter()
        //        .map(|s| s.chars().count())
        //        .sum::<usize>()
        //);

        // There's less than one screen width left to print
        // Print everything, then clear the buffers
        if self.hex_buffer.len() < self.screen_width {
            println!("{}", self.hex_buffer);
            self.hex_buffer.clear();

            self.color_buffers.iter_mut().for_each(|color_buffer| {
                color_buffer.iter().for_each(|s| print!("{}", s));
                println!();
                color_buffer.clear();
            });

        // There's more than one screen width left to print
        // Print one screen width, then chop the printed section off the buffers
        } else {
            let mut chars = 0;
            while chars < self.screen_width {
                let s = self.color_buffers[0].remove(0);
                chars += s.chars().count();
                print!("{}", s);
            }
            println!();

            println!("{}", &self.hex_buffer[..chars]);
            self.hex_buffer = self.hex_buffer.split_off(chars);

            // Print the stack tree
            for i in 1..self.color_buffers.len() {
                for _ in self.color_buffers[0].len()..self.color_buffers[i].len() {
                    let s = self.color_buffers[i].remove(0);
                    print!("{}", s);
                }
                println!();
            }
        }
        println!();
    }

    fn check_print(&mut self) {
        if self.hex_buffer.len() >= self.screen_width {
            self.print();
        }
    }

    fn write_with_color(&mut self, hex: &str, dec: &str, color_stack: &[(u8, u8, u8)]) {
        // Top up new buffers
        while self.color_buffers.len() < color_stack.len() {
            let new_buffer = if self.color_buffers.is_empty() {
                vec![]
            } else {
                self.color_buffers[0]
                    .iter()
                    .map(|s| " ".repeat(s.chars().count()).into())
                    .collect()
            };
            self.color_buffers.push(new_buffer);
        }

        let color = color_stack.iter().last().unwrap();
        let fg_color = get_contrasting_color(*color);

        write!(self.hex_buffer, "{}", hex).unwrap();

        // Fill stack buffers with color up to dec level
        self.color_buffers
            .iter_mut()
            .zip(&color_stack[..color_stack.len() - 1])
            .for_each(|(color_buffer, color)| {
                color_buffer.push(
                    " ".repeat(hex.len())
                        .on_truecolor(color.0, color.1, color.2),
                );
            });

        // Write dec values to last colour stack
        self.color_buffers[color_stack.len() - 1].push(
            dec.on_truecolor(color.0, color.1, color.2)
                .truecolor(fg_color.0, fg_color.1, fg_color.2),
        );

        // Fill remaining stack buffers with uncoloured values
        for i in color_stack.len()..self.color_buffers.len() {
            self.color_buffers[i].push(" ".repeat(dec.chars().count()).into());
        }

        self.check_print();
    }

    fn write_u8(&mut self, val: u8, color_stack: &[(u8, u8, u8)]) {
        let hex = format!("{:0>2x}", val);
        let hex = format!("{: <5}", hex);
        let dec = format!("{: <5}", val);

        self.write_with_color(&hex, &dec, color_stack);
    }

    fn write_char(&mut self, val: u8, color_stack: &[(u8, u8, u8)]) {
        let hex = format!("{:0>2x}", val);
        let hex = format!("{: <5}", hex);

        let ascii = std::ascii::escape_default(val).collect::<Vec<_>>();
        let ascii = String::from_utf8(ascii).unwrap();
        let dec = format!("{: <5}", ascii);

        self.write_with_color(&hex, &dec, color_stack);
    }

    fn write_u32(&mut self, val: u32, color_stack: &[(u8, u8, u8)]) {
        let hex = val.to_le_bytes().iter().fold(String::new(), |mut acc, x| {
            let hex = format!("{:0>2x}", x);
            write!(acc, "{: <5}", hex).unwrap();

            acc
        });

        let dec = format!("{: <20}", val);

        self.write_with_color(&hex, &dec, color_stack);
    }
}

pub fn print_horizontal(data: &Data, writer: &mut HexWriter, color_stack: &[(u8, u8, u8)]) {
    let color = (random(), random(), random());
    let color_stack = color_stack
        .iter()
        .cloned()
        .chain([color])
        .collect::<Vec<_>>();
    match data {
        Data::Primative(primative_array) => match primative_array {
            PrimativeArray::U8(items) => items.iter().for_each(|x| {
                writer.write_u8(*x, &color_stack);
            }),
            PrimativeArray::U16(items) => todo!(),
            PrimativeArray::U32(items) => items.iter().for_each(|x| {
                writer.write_u32(*x, &color_stack);
            }),

            PrimativeArray::U64(items) => todo!(),
            PrimativeArray::U128(items) => todo!(),
            PrimativeArray::Char(items) => items.iter().for_each(|x| {
                writer.write_char(*x, &color_stack);
            }),
        },
        Data::List(datas) => datas
            .iter()
            .for_each(|d| print_horizontal(d, writer, &color_stack)),
    }
}
