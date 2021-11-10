use prettytable::{cell, format, Row, Table};

use crate::Value;

pub(crate) fn to_fmt_table(rows: &Vec<Value>) -> Table {
    if rows.is_empty() {
        return Table::new();
    }
    assert!(rows[0].as_strct().is_some());
    let (_, cols) = rows[0].as_strct().unwrap();

    let mut table = Table::new();

    // Add header row
    let header_labels = Row::new(
        (*cols)
            .iter()
            .map(|(name, _)| cell![name.clone()])
            .collect(),
    );
    table.add_row(header_labels);

    // Add values
    for row in rows {
        let (_, cols) = row.as_strct().unwrap();
        let values = Row::new(
            (*cols)
                .iter()
                .map(|(_, val)| cell![val.to_string()])
                .collect(),
        );
        table.add_row(values);
    }

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table
}
