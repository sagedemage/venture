/* Physics implementations */
use crate::object;

pub fn activate_collision(player: &mut object::Player<'_>, object: &object::Object<'_>) {
    /* Player and Object collision */
    let vertex_gap: i32 = 2;
    if player.dstrect.y < object.dstrect.y + object.srcrect.h - vertex_gap && player.dstrect.y + player.srcrect.h > object.dstrect.y + vertex_gap {
        if player.dstrect.x + player.srcrect.w > object.dstrect.x && player.dstrect.x < object.dstrect.x + object.srcrect.w / 2 {
            // left collision
            player.dstrect.x = object.dstrect.x - player.srcrect.w; 
        }
        else if player.dstrect.x < object.dstrect.x + object.srcrect.w && player.dstrect.x > object.dstrect.x + object.srcrect.w / 2 {
            // right collision
            player.dstrect.x = object.dstrect.x + object.srcrect.w; 
        }
    }

    else if player.dstrect.x < object.dstrect.x - vertex_gap + object.srcrect.w && player.dstrect.x + player.srcrect.w > object.dstrect.x + vertex_gap {
        if player.dstrect.y + player.srcrect.h > object.dstrect.y && player.dstrect.y < object.dstrect.y + object.srcrect.h / 2 {
            // top collision
            player.dstrect.y = object.dstrect.y - player.srcrect.h;
        }
        else if player.dstrect.y < object.dstrect.y + object.srcrect.h && player.dstrect.y > object.dstrect.y + object.srcrect.h / 2 {
            // bottom collision
            player.dstrect.y = object.dstrect.y + object.srcrect.h;
        }    
    }
}
