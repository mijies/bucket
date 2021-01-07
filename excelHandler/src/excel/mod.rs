use std::cell::RefCell;
use std::fmt;
use std::fs::File;
use calamine::{Reader, open_workbook, Error};
use xlsxwriter::Workbook;

mod reader;
pub use reader::CellValue;


pub struct ExcelHandle {
    wb: RefCell<Wb>, // internal mutability
    path: String,
    mode: Mode,
}

enum Wb {
    Reader(reader::XlsxReader),
    Writer(reader::XlsxReader),
    Creater(Workbook),
}

#[derive(PartialEq, Debug)]
pub enum Mode {
    Read,
    Write,
    Create,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode = match self {
            Mode::Read => "Read",
            Mode::Write => "Write",
            _ => "Create",
        };
        write!(f, "{}", mode)
    }
}

impl ExcelHandle {
    pub fn new(file_path: String, mode: Mode) -> Result<Self, Error> {
        match mode {
            Mode::Read => Ok(Self {
                wb: RefCell::new(Wb::Reader(open_workbook(&file_path)?)),
                path: file_path,
                mode: mode,
            }),
            Mode::Write => Ok(Self {
                wb: RefCell::new(Wb::Writer(open_workbook(&file_path)?)),
                path: file_path,
                mode: mode,
            }),
            _ => {
                    if let Ok(_) = File::open(&file_path) {
                        return Err(Error::Msg("File with same name already exists"))
                    }
                    Ok(Self {
                    wb: RefCell::new(Wb::Creater(Workbook::new(&file_path))),
                    path: file_path,
                    mode: mode,
                })
            }
        }
    }

    // return all sheetnames
    pub fn get_sheetnames(&self) -> Vec<String> {
        match *self.wb.borrow() {
            Wb::Reader(ref wb) => wb.sheet_names().to_owned(),
            _ => vec![]
        }
    }

    // return a list of sheet names
    pub fn find_sheets<I, J>(&self,
            rows: &impl Fn() -> I,
            cols: &impl Fn() -> J,
            func: &dyn Fn(&CellValue) -> bool
        )
        -> Vec<String>
        where
            I: Iterator<Item=u32>,
            J: Iterator<Item=u32>,
    {
        let mut sheets = Vec::new();
        for sheet in self.get_sheetnames() {
            if let Some(_) = self.find_cell(sheet.as_str(), rows, cols, &func) {
                sheets.push(sheet);
            }
        }
        sheets
    }

    // return cell absolute address
    pub fn find_cell<I, J, F>(&self,
            sheetname: &str,
            rows: &impl Fn() -> I,
            cols: &impl Fn() -> J,
            func: F
        )
        -> Option<(u32, u32)> 
        where
            I: Iterator<Item=u32>,
            J: Iterator<Item=u32>,
            F: Fn(&CellValue) -> bool,
    {
        match *self.wb.borrow_mut() {
            Wb::Reader(ref mut range) => {
                reader::find_cell_reader(range, sheetname, rows, cols, func)
            }
            _ => None
        }
    }


    // retrun a vector of cell value vectors
    pub fn iterate_row_values<I, J, F>(&self,
            sheetname: &str,
            rows: &impl Fn() -> I,
            cols: &impl Fn() -> J,
            func: F
        )
        -> Vec<Vec<CellValue>>
        where
            I: Iterator<Item=u32>,
            J: Iterator<Item=u32>,
            F: Fn(&Vec<CellValue>) -> bool,
    {
        match *self.wb.borrow_mut() {
            Wb::Reader(ref mut range) => {
                reader::iterate_row_values_reader(range, sheetname, rows, cols, func)
            }
            _ => vec![]
        }
    }

    // Methods only when writable is True

    fn is_writable(&self) {
        assert_ne!(self.mode, Mode::Read, "Writable method not allowed in read-only mode");
    }

    pub fn set_range_values(&self, sheetname: &str) -> Result<(), Error> {
        self.is_writable();
        // if let Some(Ok(mut range)) = self.wb.worksheet_range(sheetname) {
        //     range.set_value((5, 5), CellValue::Float(1.0));
        // };
        Ok(())
    }

}

// wrapper function
pub fn is_writable<'a, F>(ex: &'a ExcelHandle, func: F)
    -> impl Fn(&str) -> Result<(), Error> + '_ // anonymous lifetime
    where
        F: Fn(&ExcelHandle, &str) -> Result<(), Error> + 'a // or just 'static
{
    move |x: &str| {
        println!("Wrapper func OK");
        func(ex, x)
    }
}
