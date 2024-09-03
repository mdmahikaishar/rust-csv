use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

const CSV_SEP: &str = ",";

type CsvCell = String;
type CsvHead = Vec<CsvCell>;
type CsvRow = Vec<CsvCell>;

/**
 * CSV File
 *
 * CSV file manupulation for Rust Lang.
 */
#[derive(Debug, Default)]
pub struct CsvFile {
    heads: CsvHead,
    rows: Vec<CsvRow>,
}

impl CsvFile {
    /**
     * New
     *
     * New Instance of CSV file.
     */
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * Heads
     *
     * Get headers.
     */
    pub fn heads(&self) -> &CsvHead {
        &self.heads
    }

    /**
     * Head Pos
     *
     * Head position
     */
    pub fn head_pos(&self, name: &str) -> Option<usize> {
        self.heads.iter().position(|head| head.eq(name))
    }

    /**
     * Rows
     *
     * Get rows.
     */
    pub fn rows(&self) -> &Vec<CsvRow> {
        &self.rows
    }

    /**
     * Cols
     *
     * Get all the columns.
     */
    pub fn cols(&self, name: &str) -> Option<CsvRow> {
        let index = self.head_pos(name)?;

        let cols = self
            .rows
            .iter()
            .map(|row| {
                if let Some(data) = row.get(index) {
                    data.to_string()
                } else {
                    "".to_string()
                }
            })
            .collect::<Vec<String>>();

        Some(cols)
    }

    /**
     * Row
     *
     * Get the row.
     */
    pub fn row(&self, position: usize) -> Option<&CsvRow> {
        self.rows.get(position)
    }

    /**
     * Cell
     *
     * Get the cell.
     */
    pub fn cell(&self, row: usize, col: usize) -> Option<&CsvCell> {
        let row_data = self.rows.get(row)?;

        row_data.get(col)
    }
}

impl CsvFile {
    /**
     * Push Head
     *
     * Push new head.
     */
    pub fn push_head(&mut self, name: &str) {
        self.heads.push(name.to_string());
    }

    /**
     * Set Head
     *
     * Set head to target position.
     */
    pub fn set_head(&mut self, position: usize, name: &str) {
        if let None = self.heads.get(position) {
            return;
        }

        self.heads[position] = name.to_string();
    }

    /**
     * Insert Head
     *
     * Insert new head.
     */
    pub fn insert_head(&mut self, position: usize, name: &str) {
        self.heads.insert(position, name.to_string());
    }

    /**
     * Delete Head
     *
     *
     */
    pub fn delete_head(&mut self, position: usize) -> String {
        self.heads.remove(position)
    }

    /**
     * Pop Head
     *
     *
     */
    pub fn pop_head(&mut self) -> Option<String> {
        self.heads.pop()
    }

    /**
     * Push Col
     *
     * Push new column.
     */
    pub fn push_col(&mut self, row: usize, value: &str) {
        if let None = self.rows.get(row) {
            return;
        }

        self.rows[row].push(value.to_string());
    }

    /**
     * Set Col
     *
     * Set column to target position.
     */
    pub fn set_col(&mut self, row: usize, col: usize, value: &str) {
        if let None = self.rows.get(row) {
            return;
        }

        if let None = self.rows[row].get(col) {
            return;
        }

        self.rows[row][col] = value.to_string();
    }

    /**
     * Insert Col
     *
     * Insert new column.
     */
    pub fn insert_col(&mut self, row: usize, position: usize, value: &str) {
        self.rows[row].insert(position, value.to_string())
    }

    /**
     * Delete Col
     *
     * Delete all columns.
     */
    pub fn delete_col(&mut self, position: usize) {
        self.delete_head(position);

        // Rows
        for row in self.rows.iter_mut() {
            if let None = row.get(position) {
                continue;
            }

            row.remove(position);
        }
    }

    /**
     * Pop Col
     *
     * Pop all columns.
     */
    pub fn pop_col(&mut self) {
        self.pop_head();

        // rows
        self.rows.iter_mut().for_each(|row| {
            row.pop();
        });
    }

    /**
     * Push Row
     *
     * Push new row.
     */
    pub fn push_row(&mut self, value: &[&str]) {
        self.rows.push(self.row_mapper(value));
    }

    /**
     * Set Row
     *
     * Set row to target position.
     */
    pub fn set_row(&mut self, position: usize, value: &[&str]) {
        if let None = self.rows.get(position) {
            return;
        }

        self.rows[position] = self.row_mapper(value);
    }

    /**
     * Insert Row
     *
     * Insert new row.
     */
    pub fn insert_row(&mut self, position: usize, value: &[&str]) {
        self.rows.insert(position, self.row_mapper(value));
    }

    /**
     * Delete Row
     *
     *
     */
    pub fn delete_row(&mut self, position: usize) -> Option<Vec<String>> {
        if let None = self.rows.get(position) {
            return None;
        }

        Some(self.rows.remove(position))
    }

    /**
     * Pop Row
     *
     *
     */
    pub fn pop_row(&mut self) -> Option<Vec<String>> {
        self.rows.pop()
    }

    /**
     * Row Mapper
     *
     * Maps slices into CsvRow.
     */
    fn row_mapper(&self, value: &[&str]) -> CsvRow {
        value
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
    }
}

impl CsvFile {
    /**
     * Read
     *
     * Read CSV file.
     */
    pub fn read<P: AsRef<Path>>(path: P) -> Result<CsvFile, io::Error> {
        let file = File::open(path)?;
        let buf = BufReader::new(file);

        let contents = buf
            .lines()
            .flatten()
            .filter(|line| !line.is_empty())
            .map(|row| {
                row.split(CSV_SEP)
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<CsvRow>>();

        Ok(CsvFile {
            heads: contents[0].clone(),
            rows: contents[1..].to_vec(),
        })
    }

    /**
     * Write
     *
     * Write CSV file.
     */
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        let file = File::create(path)?;
        let mut buf = BufWriter::new(file);

        if !self.heads.is_empty() {
            buf.write(self.heads().join(CSV_SEP).as_bytes())?;
            buf.write("\n".as_bytes())?;
        }

        for row in self.rows().into_iter() {
            buf.write(row.join(CSV_SEP).as_bytes())?;
            buf.write("\n".as_bytes())?;
        }

        buf.flush()
    }
}

impl fmt::Display for CsvFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // border
        for head in self.heads.iter() {
            write!(f, "- {} -", "-".repeat(head.len()))?;
        }
        write!(f, "\n")?;

        // heads
        for head in self.heads.iter() {
            write!(f, "- {} -", head)?;
        }
        write!(f, "\n")?;

        // border
        for head in self.heads.iter() {
            write!(f, "- {} -", "-".repeat(head.len()))?;
        }
        write!(f, "\n")?;

        // rows
        for row in self.rows.iter() {
            for col in row.iter() {
                write!(f, "- {} -", col)?;
            }

            write!(f, "\n")?;
        }

        write!(f, "")
    }
}
