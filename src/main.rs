use std::time::Duration;
use std::thread;
use std::thread::sleep;

use ws::*;
use dirs::{config_dir};
use regex::Regex;

mod minecraft;
use minecraft::command::*;

mod magicavoxel;
use magicavoxel::minecraft::{MinecraftVox, MinecraftBlockInfo, load_vox_from_file};

struct Location {
    x: u32,
    y: u32,
    z: u32,
}

fn build_voxel(sender: &Sender, file_name: String, base_location: &Location, direction: &str) {
    println!("build_voxel start");
    let config_dir_path = config_dir().unwrap().join("minecraft-builder-rs");
    let file_path = config_dir_path.join("vox").join(format!("{}.vox", file_name));
    let palette_file_path = config_dir_path.join("palette.json");
    if file_path.exists() && palette_file_path.exists() {
        println!("file_path found");
        let vox: MinecraftVox = load_vox_from_file(&file_path.to_str().unwrap(), palette_file_path.to_str().unwrap());
        let voxel_list: Vec<MinecraftBlockInfo> = vox.blocks;

        for voxel in voxel_list {
            println!("{:?}", voxel);
            let mut block_location = Location{
                x: base_location.x,
                y: base_location.y,
                z: base_location.z,
            };

            match direction.trim() {
                "w" => {
                    block_location.x += voxel.x;
                    block_location.y += voxel.z;
                    block_location.z += vox.size.x - voxel.y;
                },
                "e" => {
                    block_location.x += vox.size.y - voxel.x;
                    block_location.y += voxel.z;
                    block_location.z += voxel.y;
                },
                "s" => {
                    block_location.x += vox.size.y - voxel.y;
                    block_location.y += voxel.z;
                    block_location.z += vox.size.x - voxel.x;
                },
                "n" | "" | _ => {
                    block_location.x += voxel.y;
                    block_location.y += voxel.z;
                    block_location.z += voxel.x;
                }
            }
            println!("{},{},{}", block_location.x, block_location.y, block_location.z);

            let command = create_set_block_command(
                block_location.x,
                block_location.y,
                block_location.z,
                voxel.block_type, 
                "replace".to_owned(),
            );
            println!("{}", command);
            sender.send(command).unwrap();
            sleep(Duration::from_millis(100));
        }
    }
}

fn parse_and_exec_command(sender: Sender, command_message: String) {
    // parse with regexp
    let build_re = Regex::new(r"^build ([^ ]+) (\d+),(\d+),(\d+)( *[a-z]*)").unwrap();
    if build_re.is_match(&command_message) {
        let caps = build_re.captures(&command_message).unwrap();
        let voxel_name = caps.get(1).unwrap().as_str().to_owned();
        let location = Location{
            x: caps.get(2).unwrap().as_str().to_owned().parse::<u32>().unwrap(),
            y: caps.get(3).unwrap().as_str().to_owned().parse::<u32>().unwrap(),
            z: caps.get(4).unwrap().as_str().to_owned().parse::<u32>().unwrap(),
        };
        let direction = caps.get(5).unwrap().as_str();
        build_voxel(&sender, voxel_name, &location, direction);
    }
}

struct Client {
    out: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Opened");

        let player_chat_event_command = create_player_message_subscribe_command();
        println!("{}", player_chat_event_command);

        self.out.send(player_chat_event_command)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        //println!("Got message: {}", msg);

        let message: MCMessage = parse_message(msg.as_text().unwrap());
        let properties = message.body.properties.clone();
        if !&properties.is_none() && !message.body.event_name.is_none() && &properties.unwrap().sender != "外部" && &message.body.event_name.unwrap() == "PlayerMessage" {
            let sender = self.out.clone();
            let command_message = message.body.properties.unwrap().message.clone();
            thread::spawn(|| {
                parse_and_exec_command(sender, command_message);
            });
        }

        //self.out.close(CloseCode::Normal)
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);

        println!("Shutting down server after first connection closes.");
        self.out.shutdown().unwrap();
    }

    fn on_error(&mut self, err: Error) {
        println!("Shutting down server for error: {}", err);
        self.out.shutdown().unwrap();
    }
}

fn main() {
    listen("127.0.0.1:33016", |out| {
        Client {
            out,
        }
    }).unwrap();
}
