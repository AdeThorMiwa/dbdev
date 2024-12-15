use crate::{
    cursor::Cursor,
    row::{Row, RowSerializationError},
    table::Table,
};

use super::layout::{
    LEAF_NODE_CELL_SIZE, LEAF_NODE_HEADER_SIZE, LEAF_NODE_KEY_OFFSET, LEAF_NODE_KEY_SIZE,
    LEAF_NODE_MAX_CELLS, LEAF_NODE_NUM_CELLS_OFFSET, LEAF_NODE_VALUE_OFFSET, LEAF_NODE_VALUE_SIZE,
    NODE_TYPE_SIZE,
};

pub enum NodeError {
    NodeFullInsertError,
    SerializationError(RowSerializationError),
}

pub struct LeafNode {
    raw: Vec<u8>,
}

impl LeafNode {
    fn cell_count(&self) -> usize {
        let bytes: [u8; 4] = self.raw[LEAF_NODE_NUM_CELLS_OFFSET..LEAF_NODE_HEADER_SIZE]
            .try_into()
            .expect("invalid num cells");

        u32::from_be_bytes(bytes) as usize
    }

    fn get_cell(&mut self, cell_num: usize) -> &mut [u8] {
        let cell_offset = cell_num * LEAF_NODE_CELL_SIZE;
        let offset = LEAF_NODE_HEADER_SIZE + cell_offset;
        &mut self.raw[offset..(offset + LEAF_NODE_CELL_SIZE)]
    }
}

pub struct InternalNode {
    raw: Vec<u8>,
}

pub enum Node {
    Leaf(LeafNode),
    Internal(InternalNode),
}

impl From<Vec<u8>> for LeafNode {
    fn from(value: Vec<u8>) -> Self {
        Self { raw: value }
    }
}

impl From<Vec<u8>> for InternalNode {
    fn from(value: Vec<u8>) -> Self {
        Self { raw: value }
    }
}

impl From<Vec<u8>> for Node {
    fn from(page: Vec<u8>) -> Self {
        // first byte is either 0 [internal node] or 1 [leaf node]
        let node_type: [u8; NODE_TYPE_SIZE] = page[..NODE_TYPE_SIZE]
            .try_into()
            .expect("failed to extract node_type");
        let node_type = u8::from_be_bytes(node_type);
        if node_type == 0 {
            Self::Leaf(LeafNode::from(page))
        } else {
            Self::Internal(InternalNode::from(page))
        }
    }
}

impl Node {
    pub fn to_vec_mut(&mut self) -> &mut Vec<u8> {
        match self {
            Self::Leaf(n) => &mut n.raw,
            Self::Internal(n) => &mut n.raw,
        }
    }

    fn get_cell(&mut self, cell_num: usize) -> &mut [u8] {
        match self {
            Self::Leaf(n) => n.get_cell(cell_num),
            Self::Internal(_) => unreachable!("internal nodes do not have cell"),
        }
    }

    pub fn get_cell_key(&mut self, cell_num: usize) -> &mut [u8] {
        let cell = self.get_cell(cell_num);
        &mut cell[LEAF_NODE_KEY_OFFSET..LEAF_NODE_KEY_SIZE]
    }

    pub fn set_cell_key(&mut self, cell_num: usize, key: u32) {
        let cell_key = self.get_cell_key(cell_num);
        cell_key[..].copy_from_slice(&key.to_be_bytes());
    }

    pub fn get_cell_value(&mut self, cell_num: usize) -> &mut [u8] {
        let cell = self.get_cell(cell_num);
        &mut cell[LEAF_NODE_VALUE_OFFSET..LEAF_NODE_VALUE_SIZE]
    }

    pub fn get_cell_count(&self) -> usize {
        match self {
            Self::Leaf(n) => n.cell_count(),
            Self::Internal(_) => unreachable!("internal nodes do not have cell"),
        }
    }

    pub fn insert(
        &mut self,
        key: u32,
        value: &mut Row,
        table: &mut Table,
    ) -> Result<(), NodeError> {
        let cursor = Cursor::end(table);
        let cell_num = cursor.cell_num();
        let mut page = table.pager.get_page_mut(cursor.page());

        if page.cell_count() >= LEAF_NODE_MAX_CELLS {
            // TODO: Need to implement splitting a leaf node
            return Err(NodeError::NodeFullInsertError);
        }

        if cell_num < page.cell_count() {
            // Make room for new cell
            for i in (cell_num..page.cell_count()).rev() {
                let src = self.get_cell(i - 1).to_vec();
                let dest = self.get_cell(i);
                dest.copy_from_slice(&src[..]);
            }
        }

        page.increment_cell_count();
        page.set_cell_key(key, cell_num);

        let dest = page.get_cell_value(cell_num);
        value
            .serialize(dest)
            .map_err(|e| NodeError::SerializationError(e))?;

        Ok(())
    }
}
