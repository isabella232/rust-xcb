extern crate xcb;

use xcb::base::*;
use xcb::xproto::*;
use xcb::ffi::xproto::*;

use std::iter::{Iterator};

fn main() {
    let (mut conn, screen_num) = Connection::connect();

    let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();

    let window = conn.generate_id();

    let values = [
        (Cw::BACK_PIXEL, screen.white_pixel()),
        (Cw::EVENT_MASK, EventMask::EXPOSURE | EventMask::KEY_PRESS),
    ];

    create_window(&mut conn,
        COPY_FROM_PARENT as u8,
        window,
        screen.root(),
        0, 0,
        150, 150,
        10,
        WindowClass::INPUT_OUTPUT as u16,
        screen.root_visual(),
        &values);

    map_window(&mut conn,window);

    conn.flush();

    let cookie = intern_atom(&mut conn,0,"_TEST_ATOM");
    let rep_res = cookie.get_reply();
    match rep_res {
        Ok(r) => {println!("Interned Atom {}", r.atom());}
        Err(_) => { panic!("Failed to intern atom"); }
    }

    loop {
        let event = conn.wait_for_event();
        match event {
            None => { break; }
            Some(event) => {
                let r = event.base.response_type();
                if r == XCB_KEY_PRESS as u8 {
                    let key_press : &KeyPressEvent = cast_event(&event);
                    println!("Key '{}' pressed", key_press.detail());
                    break;
                }
            }
        }
    }
}
