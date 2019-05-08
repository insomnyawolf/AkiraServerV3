use crate::request::utils::*;

#[derive(Debug, Default)] // , PartialEq
pub struct MultipartFormData {
    pub multipart_form_field: Vec<MultipartFormField>,
    pub multipart_file: Vec<MultipartFile>,
    pub other: Vec<String>,
}

impl MultipartFormData {
    pub fn add(&mut self, data: String) {
        let stripped_data = &data[28..];
        if stripped_data != "\r\n" {
            let content_disposition = "Content-Disposition: form-data; ";
            if stripped_data.starts_with(content_disposition) {
                let element = MultipartFormElement::new(
                    stripped_data[content_disposition.len()..]
                        .trim_end_matches("\r\n")
                        .to_string(),
                );
                if element.is_file {
                    self.multipart_file.push(MultipartFile::new(element));
                } else {
                    self.multipart_form_field
                        .push(MultipartFormField::new(element))
                }
            } else {
                self.other.push(stripped_data.to_string());
            }
        }
    }
}

#[derive(Debug)]
pub struct MultipartFile {
    pub name: String,
    pub filename: String,
    pub content_type: String,
    pub file: Vec<u8>,
}

impl MultipartFile {
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
pub struct MultipartFormField {
    pub name: String,
    pub value: String,
}

impl MultipartFormField {
    pub fn new(element: MultipartFormElement) -> MultipartFormField {
        MultipartFormField {
            name: element.name,
            value: element.content,
        }
    }
}

//ToDO Actually parse form fields
#[derive(Debug, Default, PartialEq)]
pub struct MultipartFormElement {
    //pub data: Vec<String>,
    pub name: String,
    pub content: String,
    pub is_file: bool,
    pub filename: String,
    pub content_type: String,
    pub file: Vec<u8>,
    pub other: Vec<String>,
}

impl MultipartFormElement {
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
