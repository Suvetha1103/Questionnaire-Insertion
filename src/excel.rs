use calamine::{open_workbook_auto, Reader};
use std::collections::HashMap;

pub fn read_excel(path: &str) -> anyhow::Result<Vec<HashMap<String, String>>> {
    let mut workbook = open_workbook_auto(path)?;
    let range = workbook
        .worksheet_range("Sheet1")
        .ok_or_else(|| anyhow::anyhow!("Missing sheet"))??;

    let mut rows = range.rows();
    let headers: Vec<String> = rows.next().unwrap().iter().map(|c| c.to_string()).collect();
    println!("Headers: {:?}", headers);

    let mut output = Vec::new();
    for row in rows {
        let mut map = HashMap::new();
        for (i, cell) in row.iter().enumerate() {
            map.insert(headers[i].clone(), cell.to_string());
        }
        output.push(map);
    }
    if let Some(first_row) = output.first() {
        println!("First row: {:?}", first_row);
    }
    Ok(output)
}