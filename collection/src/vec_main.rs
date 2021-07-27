fn main() {
 
    with_Vec_as_type();
    with_Vec_as_val();
}

fn with_Vec_as_type() {

    let mut row: Vec<i32> =  Vec::new();

    row.push(1);
    row.push(2);
    row.push(3);

    for i in &row {
        println!("with_Vec_as_type : what is i {:#?}", i)
    }
}

fn with_Vec_as_val() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("with_Vec_as_val : what is i {:#?}", i)
    }
}