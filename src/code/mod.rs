use calamine:: {open_workbook, Xlsx, Range, DataType, Reader as XlsxReader};
use crate::{PATH_TO_DATA_FOLDER, CSV_PATHS};

const ROOT_COLUMN : u32 = 0;
const CACHE_COLUMN : u32 = 1;

pub struct Pair(pub String, pub String);
pub(crate) struct ProgramCachePath {
    pub(crate) root: String,
    pub(crate) cache: String,
}

impl ProgramCachePath {
    pub fn new(pair: Pair) -> ProgramCachePath {
        ProgramCachePath {root: pair.0, cache: pair.1,}
    }
}

pub(crate) fn get_cache_paths() -> Vec<ProgramCachePath> {
    let path = PATH_TO_DATA_FOLDER.to_string() + &CSV_PATHS.to_string();

    let mut result : Vec<ProgramCachePath> = Vec::new();
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    for worksheet in workbook.worksheets() {
        let range: Range<DataType> = worksheet.1;
        let vert_size = range.get_size().0;
        for row in 0..vert_size {
            get_two_record(&range, row).map(|pair| {
                result.push(ProgramCachePath::new(pair));
            });
        }
    }

    result
}

pub fn get_two_record(record: &Range<DataType>, row_index: usize) -> Option<Pair> {
    let root_path: Option<&DataType> = record.get_value((row_index as u32, ROOT_COLUMN));
    let cache_path: Option<&DataType> = record.get_value((row_index as u32, CACHE_COLUMN));
    if root_path == None || cache_path == None {return None;}
    if root_path.unwrap().to_string().len() == 0 {return None;}

    Option::Some(Pair(root_path.unwrap().to_string(), cache_path.unwrap().to_string()))
}