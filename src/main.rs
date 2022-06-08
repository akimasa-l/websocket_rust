use ws::{listen, CloseCode, Handler, Handshake, Message, Result, Sender};
fn main() {
      // WebSocketの開始
      listen("127.0.0.1:8080", |out| Server {
        out: out,
        user_name: String::new(),
    })
    .unwrap();

    struct Server {
        out: Sender,
        user_name: String,
    }
    impl Handler for Server {
        // WebSocketとのコネクション接続を開始した場合
        fn on_open(&mut self, handshake: Handshake) -> Result<()> {
            let headers: &Vec<(String, Vec<u8>)> = handshake.request.headers();
            for (k, v) in headers {
                if k == "User-Name" {
                    self.user_name = String::from_utf8(v.to_vec()).unwrap();
                }
            }
            println!("{}",self.user_name);
            return Ok(());
        }
        // メッセージを受信した場合
        fn on_message(&mut self, message: Message) -> Result<()> {
              println!("kita");
            return self.out.broadcast(message);
        }
        // WebSocketとのコネクション接続が閉じた場合
        fn on_close(&mut self, _code: CloseCode, _reason: &str) {
            
        }
    }
}
