#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::net::UdpSocket;

#[derive(Clone, serde::Serialize)]
struct GameStateMessage {
  listening: bool,
  running: bool
}

#[derive(Clone, serde::Serialize)]
struct GamePacketMessage(Vec<u8>);

#[tauri::command]
fn listen(window: tauri::Window) {
  // TODO: Need to be able to end this thread from UI
  std::thread::spawn(move || {
    window.emit("state", GameStateMessage { listening: false, running: false }).expect("failed to send");
    let socket = UdpSocket::bind("127.0.0.1:7000").expect("failed to bind port 7000");
    window.emit("state", GameStateMessage { listening: true, running: false }).expect("failed to send");
    let mut packet = [0; 324];
    let mut was_running = false;
    loop {
      socket.recv_from(&mut packet).expect("failed to receive packet");
      let running_now = packet[0] != 0;
      if was_running != running_now {
        window.emit("state", GameStateMessage { listening: true, running: running_now }).expect("failed to send");
      } else if running_now {
        window.emit("packet", GamePacketMessage(packet.into())).expect("failed to send");
      }
      was_running = running_now;
    }
  });
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![listen])
    .run(tauri::generate_context!())
    .expect("error while launching");
}
