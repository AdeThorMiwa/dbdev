use std::{
    fs::{File, OpenOptions},
    io::{Error, ErrorKind},
    os::unix::fs::FileExt,
    path::PathBuf,
    process::exit,
    sync::Mutex,
};

use crate::{page::Page, table::TABLE_MAX_PAGES};
pub const PAGER_PAGE_SIZE: usize = 4096; // 4kb per page - to correspond with fs page size

pub struct Pager {
    pages: Mutex<Vec<Page>>,
    file_len: usize,
    file: File,
    page_count: Mutex<usize>,
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

        let file_len = file_len.try_into().expect("file len is greater than usize");

        if file_len % PAGER_PAGE_SIZE != 0 {
            println!("Db file is not a whole number of pages. Corrupt file.");
            exit(1);
        }

        Ok(Self {
            pages: Mutex::new(vec![]),
            file_len,
            file: pager_file,
            page_count: Mutex::new(file_len / PAGER_PAGE_SIZE),
        })
    }

    pub fn get_page_mut(&self, page_num: usize) -> Page {
        if page_num > TABLE_MAX_PAGES {
            panic!("Tried to fetch page number out of bounds. max_page: {TABLE_MAX_PAGES} page_num: {page_num}");
        }

        let mut pages = self.pages.lock().unwrap();
        let mut page_count = self.page_count.lock().unwrap();

        if self.page_exists(page_num) {
            return pages.get(page_num).unwrap().clone();
        }

        pages.insert(page_num, Page::new());
        let mut page = pages.get(page_num).unwrap().clone();

        // REFACTOR: check if there's a math.ceil alternative to acheive this

        if page_num <= *page_count {
            let page_offset = (page_num * PAGER_PAGE_SIZE) as u64;

            match self.file.read_exact_at(&mut page.to_vec_mut(), page_offset) {
                // assumption here is that if we get `ErrorKind::UnexpectedEof` here it means the page is empty
                // so its fine
                Err(e) if e.kind() != ErrorKind::UnexpectedEof => panic!("unable to read page"),
                _ => {}
            }
        }

        if page_num >= *page_count {
            *page_count += 1;
        }

        page
    }

    pub fn get_file_len(&self) -> usize {
        self.file_len
    }

    pub fn page_exists(&self, page_num: usize) -> bool {
        self.pages.lock().unwrap().get(page_num).is_some()
    }

    pub fn flush_page(&mut self, page_num: usize) -> Result<(), PagerError> {
        if let Some(page) = self.pages.lock().unwrap().get_mut(page_num) {
            let offset = (page_num * PAGER_PAGE_SIZE) as u64;

            self.file
                .write_all_at(&page.to_vec_mut()[..PAGER_PAGE_SIZE], offset)
                .map_err(|e| PagerError::FlushFailed(e))?;
        }

        Err(PagerError::FlushInvalidPage)
    }

    pub fn get_page_count(&self) -> usize {
        *self.page_count.lock().unwrap()
    }
}
