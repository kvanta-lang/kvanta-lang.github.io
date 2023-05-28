import {is_good_text, greet} from '../pkg/game_of_life.js'

export function check_good_text(text) {
    console.log("check_good_text: " + text);
    if (text === "function(a, b)") {
        return true
    }
    return false;
}
