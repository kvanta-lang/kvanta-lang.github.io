import {is_good_text, greet} from '../pkg/game_of_life.js'

export function check_good_text() {
    let text = document.getElementById("code").value;
    let canvas = document.getElementById("drawing");
    if (is_good_text(text) == 0) {
        canvas.style.backgroundColor = "#00FF00"
        canvas.innerText = "It is a valid program!";
    } else {
        canvas.style.backgroundColor = "#FF0000"
        canvas.innerText = "It is a not valid program :(";
    }
}
