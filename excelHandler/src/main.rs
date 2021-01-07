use excelhandler::excel::{ExcelHandle, CellValue, Mode};
use excelhandler::excel::is_writable;

fn main() {

    println!("{}", Mode::Read);

    let file_path = "sample/rust_sample.xlsx".to_string();
    let ex = ExcelHandle::new(file_path, Mode::Read).expect("Failed to open file");

    println!("Method: get_sheetnames");
    let sheets = ex.get_sheetnames();
    for sheet in &sheets {
        println!("{}", sheet);
    }

    println!("Method: find_cell");
    let func_condition = |x: &CellValue| x == "foo";
    let func_type = |x: &CellValue| if x.is_string() { func_condition(x) } else { false };
    let rows = || (0..10).step_by(1);
    let cols = || (1..10);
    // let a = 0..10;
    for sheet in sheets {
        if let Some(address) = ex.find_cell(sheet.as_str(), &rows, &cols, &func_type) {
            println!("{:?}", address);
        };
    }

    println!("Method: find_sheets");
    let rows = || (0..10).step_by(1);
    let cols = || (4..10);
    let sheets = ex.find_sheets(&rows, &cols, &func_type);
    println!("{:?}", sheets);

    println!("Method: iterate_row_values");
    let rows = || (0..10).step_by(2);
    let cols = || (5..);
    let func = |x: &Vec<CellValue>| x.iter().all(|y| y.is_empty());
    let values = ex.iterate_row_values("Sheet1", &rows, &cols, func);
    for value in values {
        println!("{:?}", value);
    }

    let wrapped_func = is_writable(&ex, ExcelHandle::set_range_values);
    wrapped_func("Sheet1").unwrap();

    ex.set_range_values("Sheet1").unwrap();
}
