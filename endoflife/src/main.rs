// use endoflife::rust;

// use serde_json;

use endoflife::request::api_request_all_rust_cycles;

fn main() {
    let _all_cycles = api_request_all_rust_cycles().unwrap();
}

// fn main(){
//     let json_str= r#"{
//         "releasedate": "2024-05-02",
//         "eol": false,
//         "latest": "1.78.0",
//         "latestReleaseDate": "2024-05-02",
//         "lts": false
//     }"#;

//     let json_object = serde_json::from_str::<rust::RustSingleCycle>(json_str);

//     // println!{
//     //     "{:?}",
//     //     json_object
//     // };

//     if let Ok(data)= json_object{
//         // println!("{:#?}",data);

//         let serialized_data = serde_json::to_string_pretty(&data).unwrap();

//         println!("{}", serialized_data)
//     }else{
//         println!("Not able to parse to json. Invalid data format?");
//     }

  
// }