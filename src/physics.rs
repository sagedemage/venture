/* Physics */
use crate::object;

pub fn activate_collision(player: &mut object::Player<'_>, object: &object::Object<'_>) {
    /* Player and Object collision */
    let vertex_gap: i32 = 2;
    if player.dstrect.y < object.dstrect.y + 50 - vertex_gap && player.dstrect.y + 50 > object.dstrect.y + vertex_gap {
        if player.dstrect.x + 50 > object.dstrect.x && player.dstrect.x < object.dstrect.x + 25 {
            // left collision
            player.dstrect.x = object.dstrect.x - 50; 
        }
        else if player.dstrect.x < object.dstrect.x + 50 && player.dstrect.x > object.dstrect.x + 25 {
            // right collision
            player.dstrect.x = object.dstrect.x + 50; 
        }
    }

    else if player.dstrect.x < object.dstrect.x - vertex_gap + 50 && player.dstrect.x + 50 > object.dstrect.x + vertex_gap {
        if player.dstrect.y + 50 > object.dstrect.y && player.dstrect.y < object.dstrect.y + 25 {
            // top collision
            player.dstrect.y = object.dstrect.y - 50;
        }
        else if player.dstrect.y < object.dstrect.y + 50 && player.dstrect.y > object.dstrect.y + 25 {
            // bottom collision
            player.dstrect.y = object.dstrect.y + 50;
        }    
    }
}
