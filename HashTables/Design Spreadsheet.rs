use std::collections::HashMap;

struct Spreadsheet {
    cells: HashMap<String, i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {

    fn new(rows: i32) -> Self {
        Self {
            cells: HashMap::new(),
        }
    }
    
    fn set_cell(&mut self, cell: String, value: i32) {
        self.cells.insert(cell, value);
    }
    
    fn reset_cell(&mut self, cell: String) {
        self.cells.remove(&cell);
    }
    
    fn get_value(&self, formula: String) -> i32 {
        let mut op1 = String::new();
        let mut n1 = 0;
        let mut op2 = String::new();
        let mut n2 = 0;
        let mut plus = false;

        for ch in formula.chars().skip(1) {
            if ch == '+' {
                plus = true;
            } else if plus {
                op2.push(ch);
            } else {
                op1.push(ch);
            }
        }
        
        if let Ok(val) = op1.parse::<i32>() {
            n1 = val;
        } else if let Some(&val) = self.cells.get(&op1) {
            n1 = val;
        }

        if let Ok(val) = op2.parse::<i32>() {
            n2 = val;
        } else if let Some(&val) = self.cells.get(&op2) {
            n2 = val;
        }

        n1 + n2
    }
}

/**
 * Your Spreadsheet object will be instantiated and called as such:
 * let obj = Spreadsheet::new(rows);
 * obj.set_cell(cell, value);
 * obj.reset_cell(cell);
 * let ret_3: i32 = obj.get_value(formula);
 */