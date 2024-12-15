use crate::{
    btree::node::{Node, NodeError},
    pager::PAGER_PAGE_SIZE,
    row::Row,
    table::Table,
};

pub struct Page(Node);

impl Page {
    pub fn new() -> Self {
        Self(Node::from(vec![0; PAGER_PAGE_SIZE]))
    }

    pub fn cell_count(&self) -> usize {
        self.0.get_cell_count()
    }

    pub fn get_cell_value(&mut self, cell_num: usize) -> &mut [u8] {
        self.0.get_cell_value(cell_num)
    }

    pub fn to_vec_mut(&mut self) -> &mut Vec<u8> {
        self.0.to_vec_mut()
    }

    pub fn increment_cell_count(&mut self) {}

    pub fn set_cell_key(&mut self, key: u32, cell_num: usize) {
        self.0.set_cell_key(cell_num, key)
    }

    pub fn insert(
        &mut self,
        key: u32,
        value: &mut Row,
        table: &mut Table,
    ) -> Result<(), NodeError> {
        self.0.insert(key, value, table)
    }
}

impl Clone for Page {
    fn clone(&self) -> Self {
        todo!("page cloningin");
        Self::new()
    }
}
