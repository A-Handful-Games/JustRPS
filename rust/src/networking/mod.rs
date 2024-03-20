use axum::{response::IntoResponse, routing::get, Router};
use fastwebsockets::upgrade;
use fastwebsockets::OpCode;
use fastwebsockets::WebSocketError;
use godot::log::godot_print;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;


pub fn serve(port: usize) -> Result<(), Box<dyn std::error::Error>> {
  godot_print!("Attempting to serve...");
  let app = Router::new().route("/", get(ws_handler));
  let mut rt = Runtime::new()?;
  rt.block_on(async  {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    godot_print!("Served Successfully!");
    axum::serve(listener, app).await?;

    Ok(())
  })

}

async fn handle_client(fut: upgrade::UpgradeFut) -> Result<(), WebSocketError> {
  let mut ws = fastwebsockets::FragmentCollector::new(fut.await?);
  godot_print!("ws client");
  loop {
    let frame = ws.read_frame().await?;
    match frame.opcode {
      OpCode::Close => break,
      OpCode::Text | OpCode::Binary => {
        ws.write_frame(frame).await?;
      }
      _ => {}
    }
  }

  Ok(())
}

async fn ws_handler(ws: upgrade::IncomingUpgrade) -> impl IntoResponse {
  godot_print!("ws handled 1");
  let (response, fut) = ws.upgrade().unwrap();
  godot_print!("ws handled");
  tokio::task::spawn(async move {
    if let Err(e) = handle_client(fut).await {
      eprintln!("Error in websocket connection: {}", e);
    }
  });

  response
}