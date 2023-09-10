pub(crate) fn print_table(headers: Vec<&str>, data: Vec<Vec<&str>>) {
    // Calculate the maximum width for each column
    let col_widths: Vec<usize> = headers
        .iter()
        .enumerate()
        .map(|(i, header)| {
            let data_widths: Vec<usize> = data.iter().map(|row| row[i].len()).collect();
            let max_data_width = data_widths.into_iter().max().unwrap_or(0);
            let header_width = header.len();
            max_data_width.max(header_width)
        })
        .collect();

    // Print the headers
    for (i, header) in headers.iter().enumerate() {
        print!("{:<width$} | ", header, width = col_widths[i]);
    }
    println!();

    // Print a horizontal line to separate headers and data
    for &width in &col_widths {
        print!("{:-<width$}-+-", "", width = width);
    }
    println!();

    // Print the data rows
    for row in data {
        for (i, cell) in row.iter().enumerate() {
            print!("{:<width$} | ", cell, width = col_widths[i]);
        }
        println!();
    }
}
