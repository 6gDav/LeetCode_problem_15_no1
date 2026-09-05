# <a href="https://leetcode.com/problems/3sum/description/">15. 3Sum</a>

## 📝 Description

Given an integer array nums, return all the triplets ```[nums[i], nums[j], nums[k]]``` such that ```i != j, i != k```, and ```j != k```, and ```nums[i] + nums[j] + nums[k] == 0```.

## 🧠 How I solved the problem 

First of all, I implemented an early return statement which returns an empty list if the given list has fewer than three elements. After this, I run three nested loops, where each runs one step ahead of the previous loop's index. If the sum of the indexed elements equals zero, I add the indices to the result list.

> This algorithm takes too long to run for large numbers.

## ➗ Complexity

* **Time complexity**: *O(n^4)* - 3 nested for loop and an contains methode.
* **Space complexity**: *O(n^2)* - To store the triplets in the result.

## 📊 Benchmark

I made it in release mode for more accurate results:
```bash
cargo run --release
```

Hardware: *Apple Mac Mini M4*

### 🤏 Small Input Test

* **Execution Time**: *8.75µs*
* **Memory Delta**: *0 bytes*
* **Current Memory**: *1589248 bytes*

### 😖 Stress Test (Large Input)

* **Execution Time**: *7.446002209s*
* **Memory Delta**: *0 bytes*
* **Current Memory**: *1589248 bytes*

## ❗️LeetCode dont't accept this solution ❗️