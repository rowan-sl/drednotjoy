use sn30pro::Controller;
use std::fmt::Debug;
use std::time::Duration;
use tokio::sync::mpsc::{self, error::TryRecvError, Receiver, Sender};
use inputbot::{KeybdKey, MouseButton, MouseCursor, self};

enum MoveEvent {
    MoveDirectionUpdate(i16, i16),
    ScrollAmntUpdate(i16),
}

impl Debug for MoveEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("MoveEvent")
    }
}

#[tokio::main]
async fn main() {
    let mut ct = Controller::new(0).await.unwrap();
    let (tx, mut rx): (Sender<_>, Receiver<MoveEvent>) = mpsc::channel(50);
    let _mover_task = tokio::task::spawn_blocking(move || {
        let mut move_direction = (0, 0);
        loop {
            match rx.try_recv() {
                Ok(inst) => match inst {
                    MoveEvent::MoveDirectionUpdate(x, y) => {
                        // println!("{} {}", x, y);
                        move_direction = (y as i32, -x as i32);
                    }
                    MoveEvent::ScrollAmntUpdate(_s) => {
                        //TODO
                        // ignore for now because its NOT IMPLEMENTED REEEEEEEEEEEEEEEEe
                        // MouseWheel::scroll_ver(-s as i32);
                    }
                },
                Err(e) => {
                    if e == TryRecvError::Disconnected {
                        break;
                    }
                }
            }
            MouseCursor::move_rel(move_direction.0, move_direction.1);
            std::thread::sleep(Duration::from_millis(5))
        }
    });
    let mut w_pressed = false;
    let mut a_pressed = false;
    let mut s_pressed = false;
    let mut d_pressed = false;

    loop {
        if ct.update().await.unwrap() {
            if ct.triggers().r2.pressed_since() {
                MouseButton::LeftButton.press();
            }
            if ct.triggers().l2.pressed_since() {
                MouseButton::RightButton.press();
            }
            if ct.btn_pad().x.pressed_since() {
                MouseButton::MiddleButton.press();
            }
            if ct.triggers().r2.released_since() {
                MouseButton::LeftButton.release();
            }
            if ct.triggers().l2.released_since() {
                MouseButton::RightButton.release();
            }
            if ct.btn_pad().x.released_since() {
                MouseButton::MiddleButton.release();
            }
            if ct.btn_pad().y.pressed_since() {
                tx.send(MoveEvent::ScrollAmntUpdate(-1)).await.unwrap();
            }
            if ct.btn_pad().a.pressed() {
                tx.send(MoveEvent::ScrollAmntUpdate(1)).await.unwrap();
            }
            if ct.btn_pad().b.pressed_since() {
                KeybdKey::SpaceKey.press();
            }
            if ct.btn_pad().b.released_since() {
                KeybdKey::SpaceKey.release();
            }
            let l_joy = ct.l_joy();
            println!("{:?}", l_joy);
            if l_joy.x > 0 {
                KeybdKey::WKey.press();
                w_pressed = true;
            } else if w_pressed {
                KeybdKey::WKey.release();
                w_pressed = false;
            }
            if l_joy.x < 0 {
                KeybdKey::SKey.press();
                s_pressed = true;
            } else if s_pressed {
                KeybdKey::SKey.release();
                s_pressed = true;
            }
            if l_joy.y > 0 {
                KeybdKey::DKey.press();
                d_pressed = true;
            } else if d_pressed {
                KeybdKey::DKey.release();
                d_pressed = false;
            }
            if l_joy.y < 0 {
                KeybdKey::AKey.press();
                a_pressed = true;
            } else if a_pressed {
                KeybdKey::AKey.release();
                a_pressed = false;
            }
            if ct.heart().pressed_since() {
                KeybdKey::QKey.press();
            }
            if ct.heart().released_since() {
                KeybdKey::QKey.release();
            }
            if ct.start().pressed_since() {
                KeybdKey::CKey.press();
            }
            if ct.start().released_since() {
                KeybdKey::CKey.release();
            }
            if ct.select().pressed_since() {
                KeybdKey::EscapeKey.press();
            }
            if ct.select().released_since() {
                KeybdKey::EscapeKey.release();
            }
            tx.send(MoveEvent::MoveDirectionUpdate(
                (ct.r_joy().x as f64 / 32767f64 * 3f64) as i16,
                (ct.r_joy().y as f64 / 32767f64 * 3f64) as i16,
            ))
            .await
            .unwrap();
        }
    }
}
