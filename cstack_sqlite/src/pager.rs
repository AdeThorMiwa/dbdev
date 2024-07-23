use std::{
    fs::{File, OpenOptions},
    io::{Error, ErrorKind},
    os::unix::fs::FileExt,
    path::PathBuf,
};

use crate::table::TABLE_MAX_PAGES;
pub const PAGER_PAGE_SIZE: usize = 4096; // 4kb per page - to correspond with fs page size

pub struct Pager {
    pages: Vec<Vec<u8>>,
    len: usize,
    file: File,
}

pub enum PagerError {
    FlushInvalidPage,
    FlushFailed(Error),
}

impl<'a> Pager {
    pub fn try_new(filename: PathBuf) -> std::io::Result<Self> {
        let pager_file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(filename)?;

        let file_len = pager_file
            .metadata()
            .expect("unable to get db file metadata")
            .len();

        Ok(Self {
            pages: vec![],
            len: file_len.try_into().expect("file len is greater than usize"),
            file: pager_file,
        })
    }

    pub fn get_page_mut(&mut self, page_num: usize) -> &mut Vec<u8> {
        if page_num > TABLE_MAX_PAGES {
            panic!("Tried to fetch page number out of bounds. max_page: {TABLE_MAX_PAGES} page_num: {page_num}");
        }

        if self.page_exists(page_num) {
            return self.pages.get_mut(page_num).unwrap();
        }

        self.pages.insert(page_num, vec![0; PAGER_PAGE_SIZE]);
        let page = self.pages.get_mut(page_num).unwrap();

        // REFACTOR: check if there's a math.ceil alternative to acheive this
        let mut num_pages = self.len / PAGER_PAGE_SIZE;

        if self.len % PAGER_PAGE_SIZE > 0 {
            num_pages += 1;
        }

        if page_num <= num_pages {
            let page_offset = (page_num * PAGER_PAGE_SIZE) as u64;

            match self.file.read_exact_at(page, page_offset) {
                // assumption here is that if we get `ErrorKind::UnexpectedEof` here it means the page is empty
                // so its fine
                Err(e) if e.kind() != ErrorKind::UnexpectedEof => panic!("unable to read page"),
                _ => {}
            }
        }

        page
    }

    pub fn get_file_len(&self) -> usize {
        self.len
    }

    pub fn page_exists(&self, page_num: usize) -> bool {
        self.pages.get(page_num).is_some()
    }

    pub fn flush_page(&mut self, page_num: usize, size: usize) -> Result<(), PagerError> {
        if let Some(page) = self.pages.get(page_num) {
            let offset = (page_num * PAGER_PAGE_SIZE) as u64;

            self.file
                .write_all_at(&page[..size], offset)
                .map_err(|e| PagerError::FlushFailed(e))?;
        }

        Err(PagerError::FlushInvalidPage)
    }
}
