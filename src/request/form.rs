use crate::request::utils::*;

#[derive(Debug, Default)] // , PartialEq
/// Struct that contains all data from request forms
pub struct FormData {
    /// Form Fields via multipartform & url encoded forms
    pub form_field: Vec<FormField>,
    /// Files Uploaded via body on multipart forms
    pub multipart_file: Vec<MultipartFile>,
    /// Error/non well formatted request falback
    pub other: Vec<String>,
}

impl FormData {
    /// Adds FormField elemtents deom x-www-form-urlencoded to FormData
    pub fn add_url_encoded(&mut self, data: String) {
        let fields: Vec<&str> = data.rsplit("&").collect();
        for field in fields {
            if field.contains("=") {
                let entry: Vec<&str> = field.split("=").collect();
                self.form_field.push(FormField {
                    name: percent_encoding::percent_decode(entry[0].as_bytes())
                        .decode_utf8_lossy()
                        .to_string(),
                    value: percent_encoding::percent_decode(entry[1].as_bytes())
                        .decode_utf8_lossy()
                        .to_string(),
                });
            } else {
                self.other.push(field.to_string());
            }
        }
    }
    /// Adds Multipart elemtent to FormData
    pub fn add_multipart(&mut self, data: String, bounds: &String) {
        let elements: Vec<&str> = data.split(bounds).collect();
        for mut element_str in elements {
            element_str = &element_str.trim_end_matches("\r\n--");
            if element_str != "\r\n" || element_str != "" {
                let content_disposition = "Content-Disposition: form-data; ";
                if element_str.contains(content_disposition) {
                    let element = MultipartFormElement::new(
                        element_str.replace(content_disposition, "").to_string(),
                    );
                    if element.is_file {
                        self.multipart_file.push(MultipartFile::new(element));
                    } else {
                        self.form_field.push(FormField::new(element))
                    }
                } else {
                    if element_str != "--" && element_str != "--\r\n" {
                        self.other.push(element_str.to_string());
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
/// Structure that have the data of a Multipart File
pub struct MultipartFile {
    /// Field Id
    pub name: String,
    /// Name of the uploaded file
    pub filename: String,
    /// Kinf of file uploaded
    pub content_type: String,
    /// File contents on bytes
    pub file: Vec<u8>,
}

impl MultipartFile {
    /// Creates new MultipartFile Struct from MultipartFormElement
    pub fn new(element: MultipartFormElement) -> MultipartFile {
        MultipartFile {
            name: element.name,
            filename: element.filename,
            content_type: element.content_type,
            file: element.file,
        }
    }
}

#[derive(Debug)]
/// Structure that have the data of a FormField
pub struct FormField {
    /// Field Id **/
    pub name: String,
    /// Field Value **/
    pub value: String,
}

impl FormField {
    /// Creates new FormField Struct from MultipartFormElement
    pub fn new(element: MultipartFormElement) -> FormField {
        FormField {
            name: element.name,
            value: element.content,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
/// Structure that have each field in a MultiPartForm before processing them and splitting them on Files and Fields
pub struct MultipartFormElement {
    //pub data: Vec<String>,
    name: String,
    content: String,
    is_file: bool,
    filename: String,
    content_type: String,
    file: Vec<u8>,
    other: Vec<String>,
}

impl MultipartFormElement {
    /// Creates new MultipartFormElement Struct from raw body content (String)
    pub fn new(raw: String) -> MultipartFormElement {
        let mut element = MultipartFormElement::default();
        //element.data = ;
        let data: Vec<&str> = raw.split("\r\n\r\n").collect();

        let len = data.len();

        if len > 1 {
            let info: Vec<&str> = data.clone()[0].split("\r\n").collect();
            for current in info {
                if generate_field_string(&mut element.content_type, &current, "Content-Type: ") {
                    element.is_file = true;
                } else {
                    let inf: Vec<&str> = current.split("\"; ").collect();
                    for i in inf {
                        if generate_field_string(&mut element.name, &i, "name=\"") {
                            element.name = element.name.replace("\"", "");
                        } else if generate_field_string(&mut element.filename, &i, "filename=\"") {
                            element.filename = element.filename.replace("\"", "");
                        } else {
                            element.other.push(i.to_string());
                        }
                    }
                }
            }
        } else {
            for s in &data[2..] {
                element.other.push(s.to_string());
            }
        }
        if element.is_file {
            generate_field_vec_u8(&mut element.file, &data[1]);
        } else {
            element.content = data[1].to_string();
        }
        element
    }
}
