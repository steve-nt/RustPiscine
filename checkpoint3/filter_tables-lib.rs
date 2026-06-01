#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    /// Creates a new empty table.
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    /// Adds a new row to the table from a slice of strings.
    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }

    /// Returns a new table with only the columns where the closure returns true.
    pub fn filter_col<T: Fn(&str) -> bool>(&self, filter: T) -> Option<Self> {
        let mut indices_to_keep = Vec::new();
        let mut new_headers = Vec::new();

        // 1. Determine which columns satisfy the closure
        for (i, header) in self.headers.iter().enumerate() {
            if filter(header) {
                indices_to_keep.push(i);
                new_headers.push(header.clone());
            }
        }

        // 2. Reconstruct the body keeping only the valid indices
        let mut new_body = Vec::new();
        for row in &self.body {
            let mut new_row = Vec::new();
            for &idx in &indices_to_keep {
                // Safely get the cell in case a row is shorter than the headers
                if let Some(cell) = row.get(idx) {
                    new_row.push(cell.clone());
                }
            }
            new_body.push(new_row);
        }

        Some(Table {
            headers: new_headers,
            body: new_body,
        })
    }

    /// Returns a new table with only the rows where the selected column satisfies the closure.
    /// If the column name doesn't exist, returns None.
    pub fn filter_row<T: Fn(&str) -> bool>(&self, col_name: &str, filter: T) -> Option<Self> {
        // 1. Find the index of the target column. The `?` operator will safely return 
        //    None early if the column name doesn't exist in the headers.
        let col_idx = self.headers.iter().position(|h| h == col_name)?;

        // 2. Iterate through rows and apply the filter
        let mut new_body = Vec::new();
        for row in &self.body {
            if let Some(cell) = row.get(col_idx) {
                if filter(cell) {
                    new_body.push(row.clone());
                }
            }
        }

        Some(Table {
            headers: self.headers.clone(),
            body: new_body,
        })
    }
}