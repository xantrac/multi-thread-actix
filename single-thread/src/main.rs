use actix::prelude::*;
use std::{thread, time};

#[derive(Message)]
#[rtype(result = "Result<(), std::io::Error>")]
struct SlowMessage {
    content: String,
    display_actor: Addr<FastDisplayer>,
}

#[derive(Message)]
#[rtype(result = "Result<(), std::io::Error>")]
struct FastMessage {
    content: String,
}

struct SlowProcessor;
struct FastDisplayer;

impl Actor for SlowProcessor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("SlowProcessor is alive");
        println!("SlowProcessor thread {:?}", thread::current().id());
    }
}

impl Actor for FastDisplayer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("FastDisplayer is alive");
        println!("FastDisplayer thread {:?}", thread::current().id());
    }
}

impl Handler<SlowMessage> for SlowProcessor {
    type Result = Result<(), std::io::Error>;

    fn handle(&mut self, msg: SlowMessage, _ctx: &mut Context<Self>) -> Self::Result {
        println!("SlowMessage received");
        let mut x: i32 = 3;
        while x > 0 {
            thread::sleep(time::Duration::from_secs(1));
            println!("Processing slow message {}", x);
            x -= 1;
        }

        let mut message = msg.content;
        message.push_str("SLOW");

        msg.display_actor.do_send(FastMessage { content: message });
        Ok(())
    }
}

impl Handler<FastMessage> for FastDisplayer {
    type Result = Result<(), std::io::Error>;

    fn handle(&mut self, msg: FastMessage, _ctx: &mut Context<Self>) -> Self::Result {
        println!("FastMessage received {}", msg.content);
        Ok(())
    }
}

// This start the System Arbiter by default
#[actix_rt::main]
async fn main() {
    println!("Main thread {:?}", thread::current().id());

    let fast_addr = FastDisplayer.start();
    let slow_addr = SlowProcessor.start();

    let res = slow_addr
        .send(SlowMessage {
            display_actor: fast_addr,
            content: String::from("Banana"),
        })
        .await;

    match res {
        Ok(_) => println!("Done"),
        Err(_) => println!("Error"),
    }

    ()
}
