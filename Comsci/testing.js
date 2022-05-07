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

stack = [1, 2];

let output = stack[stack.length - 2] + stack[stack.length - 1];
stack.pop();
console.log(`after 1 - ${stack}`);
stack.pop();
console.log(`after 2 - ${stack}`);
stack.push(output);
console.log(`after 3 - ${stack}`);
