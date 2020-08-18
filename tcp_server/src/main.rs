// 引入包， 引入tcp的listener（用于监听客户端连接请求）和stream（流处理技术？），以及读写trait，服务每个接入客户端的thread

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8899").expect("unable to socket");
//启动tcp监听器，bind到本地8899端口，并将OK(T)（正常情况的返回值）中的T值赋予listener；
    
    println!("Listening on port {}", 8899);

//for循环，对于每一个客户端链接(connection)，执行以下逻辑
    for connection in listener.incoming(){ 
        match connection{
            Ok(stream) => { //若是正常返回值，则启动一个新线程服务于该stream，该线程中执行handle_client
                thread::spawn(|| { handle_client(stream);});
            }
            Err(_) =>{      //若是错误返回值，则continue丢弃该连接
                continue;
            },
        }
    }
}


fn handle_client(mut stream: TcpStream){
    println!("connection accepted");

    let mut buffer:[u8;1024] = [0;1024]; //定义一个1024大小的缓冲区

    loop{  //读取数据并回写数据
        match stream.read(&mut buffer){
            Ok(n) => {
                if n == 0 {
                    break;
                }
                if let Err(_) = stream.write(&buffer[..n]){
                    break;
                }
            }
            Err(_) => {
                break;
            }
        }
    }
    println!("disconnected")
}