use crate::table::Table;

pub struct Cursor {
    pub(self) page: usize,
    pub(self) cell: usize,
    pub(self) end_of_table: bool,
}

impl Cursor {
    pub fn start(table: &mut Table) -> Self {
        let root_page_num = table.get_root_page_num();
        let root_page = table.pager.get_page_mut(root_page_num);
        let end_of_table = root_page.cell_count() == 0;

        Self {
            page: root_page_num,
            cell: 0,
            end_of_table,
        }
    }

    pub fn end(table: &mut Table) -> Self {
        let root_page_num = table.get_root_page_num();
        let root_page = table.pager.get_page_mut(root_page_num);

        Self {
            page: root_page_num,
            cell: root_page.cell_count(),
            end_of_table: true,
        }
    }

    pub fn end_of_table(&self) -> bool {
        self.end_of_table
    }

    pub fn cell_num(&self) -> usize {
        self.cell
    }

    pub fn page(&self) -> usize {
        self.page
    }

    pub fn advance(&mut self, table: &mut Table) {
        let page = table.pager.get_page_mut(self.page);
        self.cell += 1;
        self.end_of_table = self.cell >= page.cell_count();
    }
}
