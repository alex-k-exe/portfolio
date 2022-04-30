numbers = [1, 9, 2, 3, 7, 4, 5, 10, 2, 3]

// function Bubble(nums){
//     /* Changes nums, uses too many variables*/
//     let swaps = 0;
//     let swapped = [];
//     while(true){
//         for (let i = 0; i < nums.length - 2; i++) {
//             let earlier = nums[i];
//             let later = nums[i + 1];
//             if (earlier > later){
//                  let temp = earlier;
//                  earlier = later;
//                  later = temp;
//                  swaps = 1;
//             }
//         }
//         swapped.push(nums[nums.length - 1]);
//         nums.pop();
//         if (swaps==0){
//              break;
//         }
//     }
//     nums = swaps;
//     console.log(nums);
// }

numbers = [1, 9, 2, 3, 7, 4, 5, 10, 2, 3];

function Selection(nums){
    for(let i = 0; i < nums.length; i++){
        let small = i;
        for(let j = i + 1; j < nums.length; j++){
            if(nums[j] < nums[small]){
                small = j;
            }
        }
        let temp = nums[small];
        nums[small] = nums[i]
        nums[i] = temp
    }
}

function Binary(target, nums){
    let low = 0;
    let high = nums.length - 1;
    let found = [];

    while(low <= high){
        let mid = Math.floor(low + high) / 2;
        if (nums[mid] == target){
            found.push(nums[mid]);
        } else if(nums[mid] < target){
            high = mid - 1;
        } else if(nums[mid] > target){
            low = mid + 1;
        }
    }
    return(found);
}
Binary(7, numbers);