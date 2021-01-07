use std::fs::File;
use std::io::BufReader;

use calamine::{Reader, Xlsx, DataType};

pub type CellValue = DataType;
pub type XlsxReader = Xlsx<BufReader<File>>;


pub fn find_cell_reader<I, J, F>(r: &mut XlsxReader,
        sheetname: &str,
        rows: &impl Fn() -> I,
        cols: &impl Fn() -> J,
        func: F
    )
    -> Option<(u32, u32)> 
    where
        I: Iterator<Item=u32>,
        J: Iterator<Item=u32>,
        F: Fn(&DataType) -> bool,
{
    if let Some(Ok(ref range)) = r.worksheet_range(sheetname) {
        for row in rows() {
            for col in cols() {
                if let Some(ref value) = range.get_value((row, col)) {
                    if func(value) {
                        return Some((row, col));
                    }
                }
            }
        }
    };
    None
}

pub fn iterate_row_values_reader<I, J, F>(r: &mut XlsxReader,
        sheetname: &str,
        rows: &impl Fn() -> I,
        cols: &impl Fn() -> J,
        func: F
    )
    -> Vec<Vec<DataType>>
    where
        I: Iterator<Item=u32>,
        J: Iterator<Item=u32>,
        F: Fn(&Vec<DataType>) -> bool,
{
    let mut values = Vec::new();
    if let Some(Ok(ref range)) = r.worksheet_range(sheetname) {
        for col in cols() {
            let mut vec = Vec::new();
            for row in rows() {
                if let Some(value) = range.get_value((row, col)) {
                    vec.push(value.clone()); // value is &calamine::DataType
                }
            }
            if func(&vec) { break; };
            values.push(vec);
        }
    };
    values
}