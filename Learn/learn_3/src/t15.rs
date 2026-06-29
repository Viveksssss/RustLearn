use std::time::Duration;

use futures::executor::block_on;

async fn do_something() {
    do_before().await;
    println!("go go");
}

async fn do_before() {
    println!("do_before");
}

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "大胖熊".to_string(),
        name: String::from("hhh"),
    }
}

async fn sing_song(song: Song) {
    println!("为大家献上一首由{}创作的歌曲{}", song.author, song.name);
}

async fn dance() {
    println!("唱歌情深处,跳起舞来");
}

async fn learn_and_song() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_song();
    let f2 = dance();
    futures::join!(f1, f2);
}

pub fn test1() {
    let futures = async_main();
    block_on(futures);
}
