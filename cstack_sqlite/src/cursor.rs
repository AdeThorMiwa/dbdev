use crate::table::Table;

pub struct Cursor<'a> {
    row_count: usize,
    end_of_table: bool,
    pub table: &'a mut Table,
}

impl<'a> Cursor<'a> {
    pub fn start(table: &'a mut Table) -> Self {
        let end_of_table = table.get_row_len() == 0;
        Self {
            row_count: 0,
            end_of_table,
            table,
        }
    }

    pub fn end(table: &'a mut Table) -> Self {
        let row_count = table.get_row_len();
        Self {
            table,
            row_count,
            end_of_table: true,
        }
    }

    pub fn advance(&mut self) {
        self.row_count += 1;
        self.end_of_table = self.row_count >= self.table.get_row_len()
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn end_of_table(&self) -> bool {
        self.end_of_table
    }

    pub fn get_cursor_pos(&mut self) -> &mut [u8] {
        let row_num = self.row_count();
        self.table.get_cursor_at_pos(row_num)
    }
}
