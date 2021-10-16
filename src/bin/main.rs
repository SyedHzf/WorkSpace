use notify::{watcher, RecursiveMode, Watcher};
use std::{sync::mpsc::channel,time::Duration};



fn main() -> anyhow::Result<()> {
    let (tx,rx) = channel();

    let mut watcher = 
    watcher(tx, Duration::from_secs(5))?;
    

    // Directory Path
    watcher.watch("D:/Rust",RecursiveMode::Recursive)?; 
    
    // printing the event
    while let Ok(event) =
     rx.recv(){
        println!("Working on Event{:?}", event);
    }
    
     
   Ok(()) 
}