use lopdf::{Document, Object, ObjectId};
use std::collections::BTreeMap;

use super::{Error, Result};
pub struct MergeDocument;

impl MergeDocument {
    pub fn merge_pdf_from_mem(buffers: Vec<&[u8]>) -> Result<Document> {
        let documents: Result<Vec<Document>> = buffers
            .iter()
            .map(|buffer| Document::load_mem(*buffer).map_err(|e| Error::Lopdf(e)))
            .collect();

        Self::_merge_pdf_from_documents(documents?)
    }

    fn _merge_pdf_from_file_paths(pdfs: Vec<&str>) -> Result<Document> {
        let documents: Result<Vec<Document>> = pdfs
            .iter()
            .map(|pdf| {
                let doc = Document::load(*pdf).map_err(|e| Error::Lopdf(e))?;
                Ok(doc)
            })
            .collect();

        Self::_merge_pdf_from_documents(documents?)
    }

    fn _merge_pdf_from_documents(documents: Vec<Document>) -> Result<Document> {
        // Define a starting `max_id` (will be used as start index for object_ids).
        let mut max_id = 1;
        // Collect all Documents Objects grouped by a map
        let mut documents_pages: BTreeMap<ObjectId, Object> = BTreeMap::new();
        let mut documents_objects: BTreeMap<ObjectId, Object> = BTreeMap::new();

        for mut doc in documents {
            doc.renumber_objects_with(max_id);
            max_id = doc.max_id + 1;
            let pages = Self::_collect_pages(&mut doc)?;
            documents_pages.extend(pages);
            documents_objects.extend(doc.objects.clone());
        }

        // "Catalog" and "Pages" are mandatory.
        // Recreate the main Document with "Catalog" and "Pages" objects.
        let document = Self::_recreate_objects_pages(&documents_objects, &documents_pages)?;

        Ok(document)
    }

    fn _recreate_objects_pages(
        documents_objects: &BTreeMap<ObjectId, Object>,
        documents_pages: &BTreeMap<ObjectId, Object>,
    ) -> Result<Document> {
        let mut document = Document::with_version("1.5");

        // "Catalog" and "Pages" are mandatory.
        let mut catalog_object: Option<(ObjectId, Object)> = None;
        let mut pages_object: Option<(ObjectId, Object)> = None;

        // Process all objects except "Page" type
        for (object_id, object) in documents_objects.iter() {
            // We have to ignore "Page" (as are processed later), "Outlines" and "Outline" objects.
            // All other objects should be collected and inserted into the main Document.
            match object.type_name().unwrap_or("") {
                "Catalog" => {
                    // Collect a first "Catalog" object and use it for the future "Pages".
                    catalog_object = Some((
                        if let Some((id, _)) = catalog_object {
                            id
                        } else {
                            *object_id
                        },
                        object.clone(),
                    ));
                }
                "Pages" => {
                    // Collect and update a first "Pages" object and use it for the future "Catalog"
                    // We have also to merge all dictionaries of the old and the new "Pages" object
                    match object.as_dict() {
                        Ok(dictionary) => {
                            let mut dictionary = dictionary.clone();
                            if let Some((_, ref object)) = pages_object {
                                if let Ok(old_dictionary) = object.as_dict() {
                                    dictionary.extend(old_dictionary);
                                }
                            }

                            pages_object = Some((
                                if let Some((id, _)) = pages_object {
                                    id
                                } else {
                                    *object_id
                                },
                                Object::Dictionary(dictionary),
                            ));
                        }
                        Err(e) => {
                            return Err(Error::Lopdf(e));
                        }
                    }
                }
                "Page" => {}     // Ignored, processed later and separately
                "Outlines" => {} // Ignored, not supported yet
                "Outline" => {}  // Ignored, not supported yet
                _ => {
                    document.objects.insert(*object_id, object.clone());
                }
            }
        }

        let pages_object = pages_object.ok_or(Error::PageObjectNotFound)?;

        // Iterate over all "Page" objects and collect into the parent "Pages" created before
        for (object_id, object) in documents_pages.iter() {
            match object.as_dict() {
                Ok(dictionary) => {
                    let mut dictionary = dictionary.clone();
                    dictionary.set("Parent", pages_object.0);

                    document
                        .objects
                        .insert(*object_id, Object::Dictionary(dictionary));
                }
                Err(e) => {
                    return Err(Error::Lopdf(e));
                }
            }
        }

        let catalog_object = catalog_object.ok_or(Error::CatalogObjectNotFound)?;

        // Build a new "Pages" with updated fields
        match pages_object.1.as_dict() {
            Ok(dictionary) => {
                let mut dictionary = dictionary.clone();

                // Set new pages count
                dictionary.set("Count", documents_pages.len() as u32);

                // Set new "Kids" list (collected from documents pages) for "Pages"
                dictionary.set(
                    "Kids",
                    documents_pages
                        .into_iter()
                        .map(|(object_id, _)| Object::Reference(*object_id))
                        .collect::<Vec<_>>(),
                );

                document
                    .objects
                    .insert(pages_object.0, Object::Dictionary(dictionary));
            }
            Err(e) => {
                return Err(Error::Lopdf(e));
            }
        }

        // Build a new "Catalog" with updated fields
        match catalog_object.1.as_dict() {
            Ok(dictionary) => {
                let mut dictionary = dictionary.clone();
                dictionary.set("Pages", pages_object.0);
                dictionary.remove(b"Outlines"); // Outlines not supported in merged PDFs

                document
                    .objects
                    .insert(catalog_object.0, Object::Dictionary(dictionary));
            }
            Err(e) => {
                return Err(Error::Lopdf(e));
            }
        }

        document.trailer.set("Root", catalog_object.0);

        // Update the max internal ID as wasn't updated before due to direct objects insertion
        document.max_id = document.objects.len() as u32;

        // Reorder all new Document objects
        document.renumber_objects();

        // Set any Bookmarks to the First child if they are not set to a page
        document.adjust_zero_pages();

        // Set all bookmarks to the PDF Object tree then set the Outlines to the Bookmark content map.
        if let Some(n) = document.build_outline() {
            if let Ok(x) = document.get_object_mut(catalog_object.0) {
                if let Object::Dictionary(ref mut dict) = x {
                    dict.set("Outlines", Object::Reference(n));
                }
            }
        }

        document.compress();
        Ok(document)
    }

    fn _collect_pages(doc: &mut Document) -> Result<BTreeMap<ObjectId, Object>> {
        let contents: Result<BTreeMap<ObjectId, Object>> = doc
            .get_pages()
            .into_iter()
            .map(|(_, object_id)| {
                let obj = doc
                    .get_object(object_id)
                    .map_err(|e| Error::Lopdf(e))
                    .cloned()?;
                Ok((object_id, obj))
            })
            .collect();

        contents
    }
}
