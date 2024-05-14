use crate::row::ROW_SIZE;

const PAGE_SIZE: usize = 4096; // 4kb per page - to correspond with fs page size
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE; // 14 rows per page
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Table {
    pages: Vec<Vec<u8>>,
    pub num_rows: u32,
}

impl Table {
    pub fn new() -> Self {
        Self {
            pages: vec![vec![0; PAGE_SIZE]; TABLE_MAX_PAGES],
            num_rows: 0,
        }
    }

    pub fn get_row_slot(&mut self, row_num: u32) -> &mut [u8] {
        let page_num = row_num / ROWS_PER_PAGE as u32;

        let page = self.pages.get_mut(page_num as usize).unwrap();

        let row_offset = row_num as usize % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;
        &mut page[byte_offset..]
    }

    pub fn increment_num_rows(&mut self) {
        self.num_rows += 1;
    }

    pub fn max_rows(&self) -> usize {
        TABLE_MAX_ROWS
    }
}
