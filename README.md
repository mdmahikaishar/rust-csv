# Rust CSV

![Rust](https://img.shields.io/badge/Rust-DD3515?style=for-the-badge&logo=rust&logoColor=white)

CSV file read & write using pure rust code.



## Get Started 

To get started with the CSV handling crate in Rust, you'll first need to add the `rust-csv` crate to your project. You can do this by adding it to your `Cargo.toml`:

```bash
cargo add rust-csv
```



## Features

- *CSV File Read & Write Support*: Provides functionalities for reading from and writing to CSV files.
- *High Performance*: Optimized for fast reading and writing operations.
- *Vector-Like Syntax*: Offers a simple and intuitive API for working with CSV data.
- *Memory Safe & Efficient*: Ensures safe and efficient memory management in accordance with Rust's guarantees.
- *IO Support*: Handles input and output operations for CSV files.
- *CSV File Manipulation*: Allows for easy manipulation and transformation of CSV data.



## Example

```rs
fn main() -> io::Result<()> {
    // Read CSV File.
    let csv_file = CsvFile::read("me.csv")?;

    println!("{csv_file}");


    // Get All Columns Data.
    let cols = csv_file.cols("Name").unwrap();

    println!("cols: {cols:?}");


    // Write CSV File.
    let mut new_csv = CsvFile::new();

    new_csv.push_head("Name");
    new_csv.push_head("Email");
    new_csv.push_head("Number");

    new_csv.push_row(&[
        "Md Mahi Kaishar",
        "mahikaishar@gmail.com",
        "+880-14003-14120",
    ]);

    new_csv.write("new.csv")?;

    Ok(())
}
```



## Contributing

Contributions are welcome! I would like you to contribute in this project.



## Roadmap

This project is in its early stages, and there are many missing features that need implementation. Check the [Issues](https://github.com/mdmahikaishar/rust-csv/issues) section for a list of features, enhancements, and bug fixes that are planned.



## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/mdmahikaishar/rust-csv/LICENSE) file for details.

