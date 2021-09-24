use std::fs::{File};
use std::io::{BufReader};
use std::collections::HashMap;

use dot_vox::*;
use serde::{Deserialize};

#[derive(Debug)]
pub struct MinecraftBlockInfo {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub i: u8,
    pub color: String,
    pub block_type: String,
}

#[derive(Deserialize, Debug)]
struct Palette {
    color: String,
    block: String,
}

#[derive(Debug)]
pub struct MinecraftVoxSize {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Debug)]
pub struct MinecraftVox {
    pub blocks: Vec<MinecraftBlockInfo>,
    pub size: MinecraftVoxSize,
}


fn load_palette_map(palette_file_path: &str) -> HashMap<String, String> {
    let palette_file = match File::open(palette_file_path) {
        Ok(n) => n,
        Err(err) => panic!("Palette file opening error: {:?}", err),
    };
    let palette_list: Vec<Palette> = serde_json::from_reader(BufReader::new(palette_file)).unwrap();
    let mut palette_map: HashMap<String, String> = HashMap::new();
    for palette in palette_list {
        palette_map.insert(palette.color, palette.block);
    }
    palette_map
}

pub fn load_vox_from_file(vox_file_path: &str, palette_file_path: &str) -> MinecraftVox {
    let voxel_data: DotVoxData = load(vox_file_path).unwrap();
    let palette_map = load_palette_map(palette_file_path);

    let model = &voxel_data.models[0];
    let mc_voxels = model.voxels.iter()
        .map(|v| {
            let p_index: usize = From::from(v.i);
            let color_code = voxel_data.palette[p_index];
            let color_hex = format!("{:x}", &color_code);
            let rgb = format!("{},{},{}", 
                u8::from_str_radix(&color_hex[6..8], 16).unwrap(),
                u8::from_str_radix(&color_hex[4..6], 16).unwrap(),
                u8::from_str_radix(&color_hex[2..4], 16).unwrap(),
            );
            MinecraftBlockInfo {
                x: From::from(v.x),
                y: From::from(v.y),
                z: From::from(v.z),
                i: v.i,
                color: rgb.clone(),
                block_type: palette_map.get(&rgb).unwrap().clone(),
            }
        })
        .collect();

    MinecraftVox{
        blocks: mc_voxels,
        size: MinecraftVoxSize{
            x: From::from(model.size.x),
            y: From::from(model.size.y),
            z: From::from(model.size.z),
        }
    }
}