/* Physics implementations */
use crate::object;

pub fn activate_collision(player: &mut object::Player<'_>, object: &object::Object<'_>) {
    /* Player and Object collision */
    let vertex_gap: i32 = 2;
    let player_width: i32 = player.get_srcrect().w;
    let player_height: i32 = player.get_srcrect().h;

    if player.dstrect.y < object.dstrect.y + object.srcrect.h - vertex_gap && player.dstrect.y + player.get_srcrect().h > object.dstrect.y + vertex_gap {
        if player.dstrect.x + player_width > object.dstrect.x && player.dstrect.x < object.dstrect.x + object.srcrect.w / 2 {
            // left collision
            player.dstrect.x = object.dstrect.x - player_width; 
        }
        else if player.dstrect.x < object.dstrect.x + object.srcrect.w && player.dstrect.x > object.dstrect.x + object.srcrect.w / 2 {
            // right collision
            player.dstrect.x = object.dstrect.x + object.srcrect.w; 
        }
    }

    else if player.dstrect.x < object.dstrect.x - vertex_gap + object.srcrect.w && player.dstrect.x + player_width > object.dstrect.x + vertex_gap {
        if player.dstrect.y + player_height > object.dstrect.y && player.dstrect.y < object.dstrect.y + object.srcrect.h / 2 {
            // top collision
            player.dstrect.y = object.dstrect.y - player_height;
        }
        else if player.dstrect.y < object.dstrect.y + object.srcrect.h && player.dstrect.y > object.dstrect.y + object.srcrect.h / 2 {
            // bottom collision
            player.dstrect.y = object.dstrect.y + object.srcrect.h;
        }    
    }
}
