import {compile_code, greet} from '../quanta-lang/pkg/quanta_lang.js'

export function check_good_text(text) {
    /**
    * /**
    * @typedef {Object} Field
    * @property {array[600][420]} x - all field pixels
    * maybe more props in the future
    * 
    * 
    * @param {string} text
    * @returns {Field}
    *  
    * Returns a Field object with state of every pixel in the canvas.
    */

    console.log("Compile code: " + text);
    let message = compile_code(text);
    let parts = message.split('\n');
    console.log("mu nans: " + parts[2])
    let nums = parts[2].split('|')
    console.log("nums: " + nums)
    let canvas = []
    while(nums.length) canvas.push(nums.splice(0,420));
    console.log("canvas " + canvas)
    let my_responce = {
      error_code: parseInt(parts[0]),
      error_message : parts[1],
      field : canvas
    }
    console.log(my_responce);
    console.log("Peace!");
    return my_responce;
    console.log("Compilation result:" + message)
    let color = 0xff0000;
    if (message == "0\n\n0000") {
        color = 0x00ff00;
    }

    var width = 600;
    var height = 420;
    
    // Create a new 2D array
    var array = [];
    
    // Fill the array with the green color value
    for (var i = 0; i < width; i++) {
      var row = [];
      for (var j = 0; j < height; j++) {

        row.push(color);
      }
      array.push(row);
    }
    return {
      error_code: 0,
      error_message: "",  
      field: array,
    };
}
