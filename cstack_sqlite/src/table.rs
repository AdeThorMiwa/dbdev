use crate::pager::{Pager, PagerError};
pub const TABLE_MAX_PAGES: usize = 100;

pub struct Table {
    pub pager: Pager,
    root_page_num: usize,
}

pub enum TableError {
    FlushError(PagerError),
}

impl<'a> Table {
    pub fn new(pager: Pager) -> Self {
        Self {
            pager,
            root_page_num: 0,
        }
    }

    pub fn flush_pages(&mut self) -> Result<(), TableError> {
        for page_num in 0..self.pager.get_page_count() {
            if self.pager.page_exists(page_num) {
                self.pager
                    .flush_page(page_num)
                    .map_err(|e| TableError::FlushError(e))?;
            }
        }
        Ok(())
    }

    pub fn get_root_page_num(&self) -> usize {
        self.root_page_num
    }
}
