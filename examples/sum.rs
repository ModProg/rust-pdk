#![no_main]

use extism_pdk::*;

#[derive(serde::Deserialize)]
struct Add {
    a: u32,
    b: u32,
}

#[derive(serde::Serialize)]
struct Sum {
    sum: u32,
}

#[function]
pub fn add(Json(add): Json<Add>) -> PluginResult<Json<Sum>> {
    let sum = Sum { sum: add.a + add.b };
    Ok(Json(sum))
}
