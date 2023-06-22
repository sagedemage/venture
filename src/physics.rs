/* Physics implementations */
use crate::object;

pub fn activate_collision(player: &mut object::Player<'_>, object: &object::Object<'_>) {
    /* Player and Object collision */
    let vertex_gap: i32 = 2;
    let player_width: i32 = player.get_srcrect().w;
    let player_height: i32 = player.get_srcrect().h;

    /* Object's position */
    let player_bottom_side: i32 = player.dstrect.y + player_height;
    let player_right_side: i32 = player.dstrect.x + player_width;
    let object_bottom_side: i32 = object.dstrect.y + object.srcrect.h;
    let object_right_side: i32 = object.dstrect.x + object.srcrect.w;

    if player.dstrect.y < object_bottom_side - vertex_gap && player_bottom_side > object.dstrect.y + vertex_gap {
        if player_right_side > object.dstrect.x && player.dstrect.x < object.dstrect.x + object.srcrect.w / 2 {
            // left collision
            player.dstrect.x = object.dstrect.x - player_width; 
        }
        else if player.dstrect.x < object_right_side && player.dstrect.x > object.dstrect.x + object.srcrect.w / 2 {
            // right collision
            player.dstrect.x = object_right_side; 
        }
    }

    else if player.dstrect.x < object.dstrect.x - vertex_gap + object.srcrect.w && player_right_side > object.dstrect.x + vertex_gap {
        if player_bottom_side > object.dstrect.y && player.dstrect.y < object.dstrect.y + object.srcrect.h / 2 {
            // top collision
            player.dstrect.y = object.dstrect.y - player_height;
        }
        else if player.dstrect.y < object_bottom_side && player.dstrect.y > object.dstrect.y + object.srcrect.h / 2 {
            // bottom collision
            player.dstrect.y = object_bottom_side;
        }    
    }
}
