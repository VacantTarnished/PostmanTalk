#[macro_use] extern crate rocket;

mod fairings;
mod api_keys;

use std::{fs, fs::File, io::Write};
use rocket::{http::{Status}, response::content::RawHtml};
use api_keys::{ UserKey, AdminKey };

fn extract_numbers(s: &String) -> i32 {
    let mut number = 0;
    for char in s.chars() {
        if char.is_numeric() {
            number = number * 10 + char.to_digit(10).unwrap() as i32;
        } else{
            break;
        }
    }

    number
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r"
    <!DOCTYPE html>
    <html>
        <head>
            <title>API</title>
        </head>
        <body>
            <p>API is running</p>
        </body>
    </html>
    ")
}

#[get("/hello")]
fn get_hello() -> &'static str {
    "Hello, world!"
}

#[post("/data?<name>&<id>", format = "application/json", data = "<data>")]
fn post_data(name: String, id: i32, data: String, _key: UserKey<'_>) -> Result<String, Status> {
    let mut result: Result<String, Status> = Err(Status::InternalServerError);
    let files = fs::read_dir("../data");

    if files.is_ok() {
        let files = files.unwrap();

        for file in files {
            if file.is_ok() {
                let file = file.unwrap();
                let file_name = file.file_name().into_string().unwrap().replace(".json", "");
                let file_id = extract_numbers(&file_name);

                if file_id == id {
                    return Err(Status::Conflict);
                }
            }
        }
    }

    if result == Err(Status::Conflict) {
        return result;
    }

    let file = File::create(format!("../data/{}{}.json", id, name));
    //if file.is_ok() {
        file.unwrap().write_all(data.as_bytes()).unwrap();
        result = Ok("Data saved".to_string());
    //} else {
    //    return result;
    //}

    result
}

#[get("/data/<id>")]
fn get_data(id: i32) -> Result<String, Status> {
    let mut result: Result<String, Status> = Err(Status::NotFound);
    let files = fs::read_dir("../data");

    if files.is_ok() {
        let files = files.unwrap();
        for file in files {
            if file.is_ok() {
                let file = file.unwrap();
                let file_name = file.file_name().into_string();
                let file_content = fs::read_to_string(file.path());
                
                if file_name.is_ok() && file_content.is_ok() {
                    let file_name = file_name.unwrap();
                    let file_content = file_content.unwrap();
                    if extract_numbers(&file_name) == id {
                        return Ok(file_content);
                    }
                } else {
                    result = Err(Status::InternalServerError);
                    break;
                }
            } else {
                result = Err(Status::InternalServerError);
                break;
            }
        }    
    } else {
        result = Err(Status::NotFound);
    }

    result
}

#[patch("/data/<id>", format = "application/json", data = "<data>")]
fn patch_data(id: i32, data: String, _key: UserKey<'_>) -> Result<String, Status> {
    let mut result: Result<String, Status> = Err(Status::NotFound);
    let files = fs::read_dir("../data");

    if files.is_ok() {
        let files = files.unwrap();
        for file in files {
            if file.is_ok() {
                let file = file.unwrap();
                let file_name = file.file_name().into_string();
                let file_content = fs::read_to_string(file.path());
                
                if file_name.is_ok() && file_content.is_ok() {
                    let file_name = file_name.unwrap();
                    if extract_numbers(&file_name) == id {
                        let file = File::create(file.path());
                        if file.is_ok() {
                            file.unwrap().write_all(data.as_bytes()).unwrap();
                            result = Ok("Data patched".to_string());
                        } else {
                            result = Err(Status::InternalServerError);
                        }
                        break;
                    }
                } else {
                    result = Err(Status::InternalServerError);
                    break;
                }
            } else {
                result = Err(Status::InternalServerError);
                break;
            }
        }  
    } else {
        result = Err(Status::InternalServerError);
    }

    result
}

#[delete("/data/<id>")]
fn delete_data(id: i32, _key: UserKey<'_>) -> Result<String, Status> {
    let mut result: Result<String, Status> = Err(Status::NotFound);
    let files = fs::read_dir("../data");

    if files.is_ok() {
        let files = files.unwrap();
        for file in files {
            if file.is_ok() {
                let file = file.unwrap();
                let file_name = file.file_name().into_string();
                
                if file_name.is_ok() {
                    let file_name = file_name.unwrap();
                    if extract_numbers(&file_name) == id {
                        
                        let deleted = fs::remove_file(file.path());
                        if deleted.is_ok() {
                            result = Ok("Data deleted".to_string());
                        } else {
                            result = Err(Status::InternalServerError);
                        }
                        break;
                    }
                } else {
                    result = Err(Status::InternalServerError);
                    break;
                }
            } else {
                result = Err(Status::InternalServerError);
                break;
            }
        }  
    } else {
        result = Err(Status::InternalServerError);
    }

    result
}

#[delete("/data")]
fn delete_all_data(_key: AdminKey<'_>) -> Result<String, Status> {
    let mut result: Result<String, Status> = Ok("No Data to delete".to_string());
    let files = fs::read_dir("../data");

    if files.is_ok() {
        let files = files.unwrap();
        for file in files {
            if file.is_ok() {
                let file = file.unwrap();
                let deleted = fs::remove_file(file.path());
                if deleted.is_ok() {
                    result = Ok("All data deleted".to_string());
                } else {
                    result = Err(Status::InternalServerError);
                    break;
                }
            } else {
                result = Err(Status::InternalServerError);
                break;
            }
        }  
    }

    result
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_hello, post_data, get_data, patch_data, delete_data, delete_all_data]).attach(fairings::CORS)
}
