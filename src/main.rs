#![feature(globs)]
extern crate serialize;
use std::string::String;
use self::serialize::json;

use vindinium::*;
mod vindinium;


fn main() {
    println!("Hello, world!")

    let settings = vindinium::Settings {
        key: String::from_str(""),
        url: String::from_str("")
    };

    vindinium::request(&settings, "");
    println!("{}", json::encode(&North));

    {
        let j = "{\"x\":8, \"y\":9}";
        let res: json::DecodeResult<Pos> = json::decode(j);
        match res {
            Ok(pos) => println!("{}", pos),
            Err(err) => println!("err: {}",err),
        };
    }

    {
        let j = "{\"id\":1, \"name\":\"bobby\", \"userId\":\"bobbert\", \"elo\":10, \"pos\":{\"x\":10,\"y\":80}, \"life\":50, \"gold\":700, \"mineCount\":3, \"spawnPos\":{\"x\":8,\"y\":3}, \"crashed\":false}";
        let res: json::DecodeResult<Hero> = json::decode(j);
        match res {
            Ok(hero) => println!("{}", hero),
            Err(err) => println!("err: {}",err),
        };
    }

    {
        let j = "{\"size\":4, \"tiles\":\"@1  ##[]@2  $-$2@3  []  @4##  ##$-\"}";
        let res: json::DecodeResult<Board> = json::decode(j);
        match res {
            Ok(board) => println!("{}", board),
            Err(err) => println!("err: {}",err),
        };
    }

}
