use db;
use paths;

pub fn run(query: &str, tsv: bool) -> failure::Fallible<()> {
    let db = db::DB::new(&paths::db_path()?)?;

    let rows_cell = std::cell::Cell::new(Some(vec![]));
    let cols = db.query(query, |row| {
        let display_row: Vec<String> = (0..row.column_count())
            .map(|i| row.get_raw(i))
            .map(|v| format_value(&v))
            .collect();
        let mut rows = rows_cell.replace(None).unwrap();
        rows.push(display_row);
        rows_cell.replace(Some(rows));
    })?;

    let rows = rows_cell.into_inner().unwrap();

    if tsv {
        print_tsv(&rows);
    }
    else {
        print_table(&cols, &rows);
    }

    Ok(())
}

fn print_table(cols: &[String], rows: &[Vec<String>]) {
    let widths = column_widths(&cols, &rows);

    print_row(&widths, &cols);
    let border: Vec<String> = widths.iter().map(|l| "-".repeat(*l)).collect();
    println!("{}", &border.join("-+-"));

    for row in rows {
        print_row(&widths, &row);
    }
}

fn print_tsv(rows: &[Vec<String>]) {
    for row in rows {
        println!("{}", row.join("\t"));
    }
}

fn format_value(v: &rusqlite::types::ValueRef) -> String {
    match v {
        rusqlite::types::ValueRef::Null => "null".to_string(),
        rusqlite::types::ValueRef::Integer(i) => format!("{}", i),
        rusqlite::types::ValueRef::Real(f) => format!("{}", f),
        rusqlite::types::ValueRef::Text(s) => format!("{}", s),
        rusqlite::types::ValueRef::Blob(b) => format!("{:?}", b),
    }
}

fn column_widths(cols: &[String], rows: &[Vec<String>]) -> Vec<usize> {
    let mut max_widths: Vec<usize> = cols.iter().map(|s| s.len()).collect();
    for row in rows {
        for (i, col) in row.iter().enumerate() {
            if col.len() > max_widths[i] {
                max_widths[i] = col.len();
            }
        }
    }
    max_widths
}

fn print_row(widths: &[usize], row: &[String]) {
    let fixed_width_row: Vec<String> = row
        .iter()
        .zip(widths.iter())
        .map(|(s, width)| format!("{:width$}", s, width=width))
        .collect();
    println!("{}", &fixed_width_row.join(" | "));
}
