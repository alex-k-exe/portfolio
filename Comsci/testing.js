const exactMath = require("exact-math");

// numbers = [1, 9, 2, 3, 7, 4, 5, 10, 2, 3];

// // Bubble sort
// function Bubble(nums) {
//     let end = nums.length - 1;
//     while (end >= -1) {
//         for (let i = 1; i >= end; i++) {
//             let before = nums[i - 1];
//             let after = nums[i];
//             if (before > after) {
//                 let temp = after;
//                 after = before;
//                 before = temp;
//                 swaps = 1;
//             }
//             end--;
//         }
//         console.log(nums);
//     }
//     console.log(nums);
// }

var userInput = "4 2 3 + *"
var inputs = userInput.split(" ");
var prev_calc = 0;
var calc = 0;

for (i = 2; i < inputs.length; i++) {
    if (isNaN(inputs[i]) = false) {
        switch (inputs[i]){
            case "+":
                calc = inputs[i - 1] + inputs[i - 2];
                break
            case "-":
                calc = inputs[i - 1] - inputs[i - 2];
                break
            case "*":
                calc = inputs[i - 1] * inputs[i - 2];
                break
            case "/":
                calc = inputs[i - 1] / inputs[i - 2];
                break
        inputs.pop()
        }
    } else if(s){

    }
}

console.log("This is your input: ${input}");
