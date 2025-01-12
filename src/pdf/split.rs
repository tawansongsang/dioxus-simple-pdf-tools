use super::error::{Error, Result};
use lopdf::Document;
use regex::Regex;

pub struct SplitDocumnet;

impl SplitDocumnet {
    pub fn split_pdf_from_mem(
        buffer: &[u8],
        split_page_numbers_str: &str,
    ) -> Result<Vec<(Document, String)>> {
        let document: Result<Document> = Document::load_mem(buffer).map_err(|e| Error::Lopdf(e));

        Self::_split_pdf(&document?, split_page_numbers_str)
    }

    pub fn split_pdf_from_mem_fixed_page(
        buffer: &[u8],
        split_fixed_page: u32,
    ) -> Result<Vec<(Document, String)>> {
        let document: Result<Document> = Document::load_mem(buffer).map_err(|e| Error::Lopdf(e));

        Self::_split_pdf_fixed_page(&document?, split_fixed_page)
    }

    fn _split_pdf_fixed_page(
        doc: &Document,
        split_fixed_page: u32,
    ) -> Result<Vec<(Document, String)>> {
        let max_pages = doc.get_pages().len() as u32;
        let split_page_numbers =
            Self::_fixed_to_split_page_numbers_u32(max_pages, split_fixed_page)?;
        let delete_page_numbers =
            Self::_convert_split_to_delete_page_numbers_u32(split_page_numbers, max_pages)?;

        let mut new_split_pdfs: Vec<(Document, String)> = Vec::new();
        let split_str: Vec<String> =
            Self::_fixed_to_split_page_numbers_str(max_pages, split_fixed_page)?;
        for (idx, delete_page_number) in delete_page_numbers.iter().enumerate() {
            let mut document = doc.clone();
            document.delete_pages(&delete_page_number);
            new_split_pdfs.push((document, split_str[idx].clone()));
        }

        Ok(new_split_pdfs)
    }

    fn _split_pdf(doc: &Document, split_page_numbers_str: &str) -> Result<Vec<(Document, String)>> {
        let max_pages = doc.get_pages().len() as u32;
        let split_page_numbers =
            Self::_string_to_split_page_numbers_u32(split_page_numbers_str, max_pages)?;
        let delete_page_numbers =
            Self::_convert_split_to_delete_page_numbers_u32(split_page_numbers, max_pages)?;
        let mut new_split_pdfs: Vec<(Document, String)> = Vec::new();
        let split_str: Vec<&str> = split_page_numbers_str.split(",").collect();
        for (idx, delete_page_number) in delete_page_numbers.iter().enumerate() {
            let mut document = doc.clone();
            document.delete_pages(delete_page_number);
            new_split_pdfs.push((document, split_str[idx].to_string()));
        }

        Ok(new_split_pdfs)
    }

    fn _fixed_to_split_page_numbers_u32(
        max_pages: u32,
        split_fixed_page: u32,
    ) -> Result<Vec<Vec<u32>>> {
        if max_pages < split_fixed_page {
            return Err(Error::FiexedPageNumberOverFlow);
        }

        let mut split_page_numbers: Vec<Vec<u32>> = Vec::new();
        let mut page_numbers: Vec<u32> = Vec::new();
        for i in 1..max_pages + 1 {
            if i % split_fixed_page == 0 {
                page_numbers.push(i);
                split_page_numbers.push(page_numbers);
                page_numbers = Vec::new();
            } else {
                page_numbers.push(i);
            }
        }

        if !page_numbers.is_empty() {
            split_page_numbers.push(page_numbers);
        }
        Ok(split_page_numbers)
    }

    fn _fixed_to_split_page_numbers_str(
        max_pages: u32,
        split_fixed_page: u32,
    ) -> Result<Vec<String>> {
        if max_pages < split_fixed_page {
            return Err(Error::FiexedPageNumberOverFlow);
        }
        let mut split_page_numbers: Vec<String> = Vec::new();
        let max_loop = max_pages / split_fixed_page;
        let is_remainder = max_pages % split_fixed_page != 0;
        for start in 0..max_loop {
            let start = (start * split_fixed_page) + 1;
            let end = start + split_fixed_page - 1;
            split_page_numbers.push(format!("{start}-{end}"));
        }
        if is_remainder {
            let start = (max_loop * split_fixed_page) + 1;
            let end = max_pages;
            split_page_numbers.push(format!("{start}-{end}"));
        }

        Ok(split_page_numbers)
    }

    fn _convert_split_to_delete_page_numbers_u32(
        split_page_numbers: Vec<Vec<u32>>,
        max_pages: u32,
    ) -> Result<Vec<Vec<u32>>> {
        let mut delete_page_numbers: Vec<Vec<u32>> = Vec::new();
        split_page_numbers.iter().for_each(|page_numbers| {
            let mut delete_page_number: Vec<u32> = Vec::new();
            let all_pages = 1..(max_pages + 1);
            all_pages.for_each(|page| {
                if !page_numbers.contains(&page) {
                    delete_page_number.push(page);
                }
            });
            delete_page_numbers.push(delete_page_number);
        });

        Ok(delete_page_numbers)
    }

    fn _string_to_split_page_numbers_u32(
        split_page_numbers_str: &str,
        max_pages: u32,
    ) -> Result<Vec<Vec<u32>>> {
        let check = Self::is_valid_string_split_pages(split_page_numbers_str)?;
        if check {
            let mut split_page_numbers: Vec<Vec<u32>> = Vec::new();
            let split_str: Vec<&str> = split_page_numbers_str.split(",").collect();
            for ss in split_str {
                let split_page_number = Self::_create_page_numbers_u32(ss, max_pages)?;
                split_page_numbers.push(split_page_number);
            }
            return Ok(split_page_numbers);
        }
        Err(Error::InValidPageNumbers)
    }

    fn _create_page_numbers_u32(split_page_number: &str, max_pages: u32) -> Result<Vec<u32>> {
        let mut page_numbers: Vec<u32> = Vec::new();
        if split_page_number.contains("-") {
            let range_nums: Vec<u32> = split_page_number
                .split("-")
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let first = range_nums[0];
            let last = range_nums[1];
            if first > last {
                return Err(Error::InValidPageNumbers);
            }
            if last > max_pages {
                return Err(Error::PageNumberOverFlow);
            }
            (first..last + 1).for_each(|page_number| page_numbers.push(page_number));
        } else {
            let page_number = split_page_number
                .trim()
                .parse::<u32>()
                .map_err(|_e| Error::InValidPageNumbers)?;
            if page_number > max_pages {
                return Err(Error::PageNumberOverFlow);
            }
            page_numbers.push(page_number);
        }

        Ok(page_numbers)
    }

    pub fn is_valid_string_split_pages(string_split_pages: &str) -> Result<bool> {
        if string_split_pages.is_empty() {
            return Err(Error::SplitPagesStrIsEmpty);
        }
        let re = Regex::new(r"(^((\d+-\d+)|\d+)(?: *, *((\d+-\d+)|\d+))*)$")
            .map_err(|_e| Error::CannotCreateRegex)?;
        let check = re.is_match(string_split_pages);
        Ok(check)
    }

    pub fn is_valid_string_split_fixed_pages(string_split_fixed_page: &str) -> Result<bool> {
        if string_split_fixed_page.is_empty() {
            return Err(Error::SplitPagesStrIsEmpty);
        }
        let re = Regex::new(r"^\d+$").map_err(|_e| Error::CannotCreateRegex)?;
        let check = re.is_match(string_split_fixed_page);
        Ok(check)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_string_split_pages_ok_1() {
        let split_pages = "";
        let result = SplitDocumnet::is_valid_string_split_pages(split_pages);
        assert!(result.is_err());
    }
    #[test]
    fn is_valid_string_split_pages_ok_2() {
        let split_pages = "1";
        let result = SplitDocumnet::is_valid_string_split_pages(split_pages);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expected = true;
        assert_eq!(expected, result);
    }
    #[test]
    fn is_valid_string_split_pages_ok_3() {
        let split_pages = "1, 5";
        let result = SplitDocumnet::is_valid_string_split_pages(split_pages);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expected = true;
        assert_eq!(expected, result);
    }
    #[test]
    fn is_valid_string_split_pages_ok_4() {
        let split_pages = "1, 2-3, 5";
        let result = SplitDocumnet::is_valid_string_split_pages(split_pages);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expected = true;
        assert_eq!(expected, result);
    }
    #[test]
    fn is_valid_string_split_pages_ok_5() {
        let split_pages = "D";
        let result = SplitDocumnet::is_valid_string_split_pages(split_pages);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expected = false;
        assert_eq!(expected, result);
    }
    #[test]
    fn is_valid_string_split_pages_ok_6() {
        let split_pages = "1D";
        let result = SplitDocumnet::is_valid_string_split_pages(split_pages);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expected = false;
        assert_eq!(expected, result);
    }
    #[test]
    fn is_valid_string_split_pages_ok_7() {
        let split_pages = "1-D";
        let result = SplitDocumnet::is_valid_string_split_pages(split_pages);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expected = false;
        assert_eq!(expected, result);
    }
    #[test]
    fn is_valid_string_split_pages_ok_8() {
        let split_pages = "1-2D";
        let result = SplitDocumnet::is_valid_string_split_pages(split_pages);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expected = false;
        assert_eq!(expected, result);
    }
    #[test]
    fn is_valid_fixed_to_split_page_number_str_ok_1() {
        let max_pages = 5;
        let split_fixed_page = 2;
        let result = SplitDocumnet::_fixed_to_split_page_numbers_str(max_pages, split_fixed_page);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expected = vec!["1-2", "3-4", "5-5"];
        assert_eq!(expected, result);
    }
    #[test]
    fn is_valid_fixed_to_split_page_number_str_ok_2() {
        let max_pages = 6;
        let split_fixed_page = 2;
        let result = SplitDocumnet::_fixed_to_split_page_numbers_str(max_pages, split_fixed_page);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expected = vec!["1-2", "3-4", "5-6"];
        assert_eq!(expected, result);
    }
    #[test]
    fn is_valid_fixed_to_split_page_number_str_ok_3() {
        let max_pages = 3;
        let split_fixed_page = 1;
        let result = SplitDocumnet::_fixed_to_split_page_numbers_str(max_pages, split_fixed_page);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expected = vec!["1-1", "2-2", "3-3"];
        assert_eq!(expected, result);
    }
    #[test]
    fn is_valid_fixed_to_split_page_number_str_ok_4() {
        let max_pages = 7;
        let split_fixed_page = 4;
        let result = SplitDocumnet::_fixed_to_split_page_numbers_str(max_pages, split_fixed_page);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expected = vec!["1-4", "5-7"];
        assert_eq!(expected, result);
    }
    #[test]
    fn is_valid_fixed_to_split_page_number_str_ok_5() {
        let max_pages = 10;
        let split_fixed_page = 4;
        let result = SplitDocumnet::_fixed_to_split_page_numbers_str(max_pages, split_fixed_page);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expected = vec!["1-4", "5-8", "9-10"];
        assert_eq!(expected, result);
    }
    #[test]
    fn is_valid_fixed_to_split_page_number_str_ok_6() {
        let max_pages = 1;
        let split_fixed_page = 1;
        let result = SplitDocumnet::_fixed_to_split_page_numbers_str(max_pages, split_fixed_page);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expected = vec!["1-1"];
        assert_eq!(expected, result);
    }

    #[test]
    fn is_valid_fixed_to_split_page_number_str_ok_7() {
        let max_pages = 5;
        let split_fixed_page = 5;
        let result = SplitDocumnet::_fixed_to_split_page_numbers_str(max_pages, split_fixed_page);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expected = vec!["1-5"];
        assert_eq!(expected, result);
    }
    #[test]
    fn is_valid_fixed_to_split_page_number_str_ok_8() {
        let max_pages = 2;
        let split_fixed_page = 4;
        let result = SplitDocumnet::_fixed_to_split_page_numbers_str(max_pages, split_fixed_page);
        assert!(result.is_err());
    }
}
