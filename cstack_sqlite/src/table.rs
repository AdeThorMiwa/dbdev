use crate::{
    pager::{Pager, PagerError, PAGER_PAGE_SIZE},
    row::ROW_SIZE,
};

const TABLE_ROWS_PER_PAGE: usize = PAGER_PAGE_SIZE / ROW_SIZE; // 14 rows per page
const TABLE_MAX_ROWS: usize = TABLE_ROWS_PER_PAGE * TABLE_MAX_PAGES;
pub const TABLE_MAX_PAGES: usize = 100;

pub struct Table {
    pager: Pager,
    num_rows: usize,
}

pub enum TableError {
    FlushError(PagerError),
}

impl<'a> Table {
    pub fn new(pager: Pager) -> Self {
        let num_rows = pager.get_file_len() / ROW_SIZE;
        Self { pager, num_rows }
    }

    pub fn get_row_slot(&mut self, row_num: usize) -> &mut [u8] {
        let page_num = row_num / TABLE_ROWS_PER_PAGE;

        let page = self.pager.get_page_mut(page_num);

        let row_offset = row_num % TABLE_ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;
        &mut page[byte_offset..]
    }

    pub fn increment_num_rows(&mut self) {
        self.num_rows += 1;
    }

    pub fn max_rows(&self) -> usize {
        TABLE_MAX_ROWS
    }

    pub fn get_row_len(&self) -> usize {
        self.num_rows
    }

    fn get_full_pages_count(&self) -> usize {
        self.num_rows / TABLE_ROWS_PER_PAGE
    }

    fn partial_page_row_count(&self) -> usize {
        self.num_rows % TABLE_ROWS_PER_PAGE
    }

    fn has_partial_page(&self) -> bool {
        self.partial_page_row_count() > 0
    }

    pub fn flush_pages(&mut self) -> Result<(), TableError> {
        let full_pages = self.get_full_pages_count();

        for page_num in 0..full_pages {
            if !self.pager.page_exists(page_num) {
                continue;
            }

            self.pager
                .flush_page(page_num, PAGER_PAGE_SIZE)
                .map_err(|e| TableError::FlushError(e))?;
        }

        // flush partial pages
        if self.has_partial_page() {
            let partial_page_num = full_pages;
            if self.pager.page_exists(partial_page_num) {
                let page_size = self.partial_page_row_count() * ROW_SIZE;

                self.pager
                    .flush_page(partial_page_num, page_size)
                    .map_err(|e| TableError::FlushError(e))?;
            }
        }

        Ok(())
    }
}
