// #[derive(Copy)]
pub struct Page(*mut [u8; PAGE_SIZE]);

type Pages = [Option<Page>; TABLE_MAX_PAGES];

use crate::simple;

pub const PAGE_SIZE: usize = 4096;
pub const ROWS_PER_PAGE: usize = PAGE_SIZE / simple::size::ALL;
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;
pub const TABLE_MAX_PAGES: usize = 100;

pub struct Table {
    num_rows: u32,
    pages: Pages,
}

extern "C" {
    fn malloc(bytes: usize) -> *mut ();
    fn free(ptr: *mut ());
}

impl Page {
    pub fn new() -> Self {
        Self(Box::into_raw(Box::new([0; PAGE_SIZE])))
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        unsafe { _ = Box::from_raw(self.0); }
        // self.0
    }
}

// macro_rules! PAGE_INIT{
//     ($val:expr, $count:expr) => {[PAGE_INIT!($val, $count, $count)]};
//     ($val:expr, $count:expr, 0) => {};
//     ($val:expr, $count:expr, $remain:expr) => $val, PAGE_INIT!($val, $expr - 1);

// }

macro_rules! PAGE_INIT {
    (@acc (0, $($_es:expr), *) -> ($($body:tt)))
        => {PAGE_INIT!(@as_expr [$($body)*])};
    (@acc ($remain:expr, $($es:expr), *) -> ($($body:tt)))
        => {PAGE_INIT!(@acc (($remain - 1), $($es),*) -> ($($body)* $($es,)*))};
    (@as_expr $e:expr) => {$e};
    [$e:expr, $n:expr] => {PAGE_INIT!(@acc ($n, $e) -> ())};
}

impl Table {
    pub fn new() -> Self {
        Self {
            num_rows: 0,
            // pages: [None; TABLE_MAX_PAGES]
            pages: PAGE_INIT!(None, TABLE_MAX_PAGES)
        }
    }
    pub fn row_slot(&mut self, row_num: u32) -> Option<&mut Page> {
        let page_num = row_num as usize / ROWS_PER_PAGE;
        let page = self.pages.get_mut(page_num)?;
        match page {
            Some(ref mut p) => Some(p),
            None => {

                unimplemented!()
            }
        }
    }
}
