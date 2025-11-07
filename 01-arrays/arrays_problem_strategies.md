# 🎯 Array Problem Solving Guide

Strategies and patterns for array/vector problems.

---

## 🧠 **Mental Model:**

1. **Can I solve it in ONE pass?** → Use hash map or two pointers
2. **Do I need to track positions?** → Use hash map with indices
3. **Is the array sorted?** → Binary search or two pointers
4. **Can I sort it?** → Often simplifies the problem
5. **Do I need to find subarrays?** → Sliding window or prefix sum
6. **Multiple elements interaction?** → Two/Three pointers

---

## 🔥 **Core Patterns**

### **1. Hash Map (Frequency/Lookup)**

**When:** Need O(1) lookup, track seen elements, or count frequencies

**Template:**

```rust
use std::collections::HashMap;

pub fn solve(nums: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        // Check if complement/target exists
        if let Some(&j) = map.get(&(target - num)) {
            return vec![j, i];
        }
        map.insert(num, i);
    }

    vec![]
}
```

**Use Cases:**

- Two Sum (find pair with target)
- Contains Duplicate (track seen)
- First Unique Character
- Group Anagrams

**Key Points:**

- `HashMap<value, index>` for position tracking
- `HashMap<value, count>` for frequencies
- Check existence with `map.contains_key()` or `map.get()`

---

### **2. Two Pointers**

**When:** Sorted array, find pairs, reverse, remove duplicates

**Template (Opposite Ends):**

```rust
pub fn two_pointer_opposite(nums: &mut [i32]) -> Vec<i32> {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        // Process current pair
        let sum = nums[left] + nums[right];

        if sum == target {
            return vec![left as i32, right as i32];
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    vec![]
}
```

**Template (Same Direction):**

```rust
pub fn two_pointer_same(nums: &mut [i32]) -> i32 {
    let mut slow = 0;

    for fast in 0..nums.len() {
        if nums[fast] != 0 {  // Some condition
            nums[slow] = nums[fast];
            slow += 1;
        }
    }

    slow as i32
}
```

**Use Cases:**

- Two Sum II (sorted array)
- Remove Duplicates
- Move Zeroes
- Container With Most Water
- Reverse Array

**Key Points:**

- **Opposite ends:** When array is sorted, finding pairs
- **Same direction (slow/fast):** In-place modifications
- Always check `left < right` to avoid infinite loops

---

### **3. Sliding Window**

**When:** Contiguous subarray/substring problems (max/min sum, length)

**Template (Fixed Size):**

```rust
pub fn sliding_window_fixed(nums: &[i32], k: usize) -> i32 {
    // Initial window
    let mut window_sum = nums[..k].iter().sum::<i32>();
    let mut max_sum = window_sum;

    // Slide the window
    for i in k..nums.len() {
        window_sum += nums[i] - nums[i - k];
        max_sum = max_sum.max(window_sum);
    }

    max_sum
}
```

**Template (Variable Size):**

```rust
pub fn sliding_window_variable(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut sum = 0;
    let mut min_len = usize::MAX;

    for right in 0..nums.len() {
        sum += nums[right];

        // Shrink window while valid
        while sum >= target {
            min_len = min_len.min(right - left + 1);
            sum -= nums[left];
            left += 1;
        }
    }

    if min_len == usize::MAX { 0 } else { min_len as i32 }
}
```

**Use Cases:**

- Maximum Sum Subarray of Size K
- Minimum Size Subarray Sum
- Longest Substring Without Repeating
- Fruits Into Baskets

**Key Points:**

- **Fixed size:** Add new, remove old (window size = k)
- **Variable size:** Expand with `right`, shrink with `left`
- Track window state (sum, count, frequencies)

---

### **4. Prefix Sum**

**When:** Need range sum queries, subarray sums

**Template:**

```rust
pub fn prefix_sum(nums: &[i32]) -> Vec<i32> {
    let mut prefix = vec![0; nums.len() + 1];

    for i in 0..nums.len() {
        prefix[i + 1] = prefix[i] + nums[i];
    }

    // Range sum [left, right] = prefix[right+1] - prefix[left]
    prefix
}
```

**With HashMap (Subarray Sum = K):**

```rust
use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut map = HashMap::new();
    map.insert(0, 1);  // Empty prefix

    let mut sum = 0;
    let mut count = 0;

    for num in nums {
        sum += num;

        if let Some(&freq) = map.get(&(sum - k)) {
            count += freq;
        }

        *map.entry(sum).or_insert(0) += 1;
    }

    count
}
```

**Use Cases:**

- Range Sum Query
- Subarray Sum Equals K
- Contiguous Array
- Product of Array Except Self (prefix + suffix)

**Key Points:**

- Build prefix array: `prefix[i] = prefix[i-1] + nums[i-1]`
- Range `[L, R]` sum = `prefix[R+1] - prefix[L]`
- Use HashMap for "sum equals target" problems

---

### **5. Sorting First**

**When:** Problem becomes easier with sorted data

**Template:**

```rust
pub fn solve_with_sort(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();  // O(n log n)

    // Now use two pointers, binary search, etc.
    two_pointer_logic(&mut nums)
}
```

**Use Cases:**

- 3Sum, 4Sum
- Merge Intervals
- Meeting Rooms
- Contains Duplicate (sort then check neighbors)

**Key Points:**

- `sort()` - stable sort
- `sort_unstable()` - faster, not stable
- `sort_by()` - custom comparator
- After sorting, many problems become O(n) with two pointers

---

### **6. Binary Search**

**When:** Sorted array, find target/position in O(log n)

**Template (Find Target):**

```rust
pub fn binary_search(nums: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    -1
}
```

**Template (Find Boundary):**

```rust
pub fn lower_bound(nums: &[i32], target: i32) -> usize {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}
```

**Use Cases:**

- Search in Sorted Array
- Find First/Last Position
- Search in Rotated Array
- Square Root

**Key Points:**

- **Inclusive bounds:** `left <= right`, return `-1`
- **Exclusive right:** `left < right`, return `left`
- Avoid overflow: `mid = left + (right - left) / 2`

---

### **7. In-Place Modification**

**When:** Space constraint O(1), modify array itself

**Template:**

```rust
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut write = 0;

    // Move non-zero elements forward
    for read in 0..nums.len() {
        if nums[read] != 0 {
            nums[write] = nums[read];
            write += 1;
        }
    }

    // Fill rest with zeros
    for i in write..nums.len() {
        nums[i] = 0;
    }
}
```

**Use Cases:**

- Remove Element
- Move Zeroes
- Sort Colors (Dutch Flag)
- Remove Duplicates

**Key Points:**

- Use read/write pointers
- Write pointer tracks valid position
- Read pointer scans entire array

---

## 🎯 **Decision Tree**

```
Is array SORTED?
├─ Yes → Binary Search or Two Pointers
└─ No
   ├─ Can I sort it?
   │  ├─ Yes → Sort first, then solve
   │  └─ No → Continue
   │
   ├─ Need O(1) lookup?
   │  └─ Yes → HashMap
   │
   ├─ Contiguous subarray?
   │  └─ Yes → Sliding Window or Prefix Sum
   │
   ├─ In-place modification?
   │  └─ Yes → Two Pointers (same direction)
   │
   └─ Otherwise → Consider brute force, optimize later
```

---

## ⚡ **Time Complexity Quick Guide**

| Pattern        | Time       | Space | Best For              |
| -------------- | ---------- | ----- | --------------------- |
| Hash Map       | O(n)       | O(n)  | Lookup, frequency     |
| Two Pointers   | O(n)       | O(1)  | Sorted, pairs         |
| Sliding Window | O(n)       | O(1)  | Subarrays             |
| Prefix Sum     | O(n)       | O(n)  | Range queries         |
| Sorting        | O(n log n) | O(1)  | Making problem easier |
| Binary Search  | O(log n)   | O(1)  | Sorted array          |

---

## 🚨 **Common Mistakes to Avoid**

### **1. Index Out of Bounds**

```rust
// ❌ Dangerous
if i < nums.len() - 1 {  // Fails if len = 0

// ✅ Safe
if i + 1 < nums.len() {
```

### **2. Integer Overflow in Mid**

```rust
// ❌ Can overflow
let mid = (left + right) / 2;

// ✅ Safe
let mid = left + (right - left) / 2;
```

### **3. Forgetting to Sort**

```rust
// ❌ Two pointers on unsorted array
let result = two_sum_sorted(&nums, target);

// ✅ Sort first
nums.sort_unstable();
let result = two_sum_sorted(&nums, target);
```

### **4. Modifying While Iterating**

```rust
// ❌ Borrow checker error
for (i, &num) in nums.iter().enumerate() {
    nums.push(num);  // Can't mutate while borrowing
}

// ✅ Collect first, then modify
let to_add: Vec<_> = nums.iter().cloned().collect();
nums.extend(to_add);
```

### **5. Using Wrong Pointer Logic**

```rust
// ❌ Infinite loop
while left <= right {
    // Forgot to move pointers!
}

// ✅ Always move pointers
while left <= right {
    if condition {
        left += 1;
    } else {
        right -= 1;
    }
}
```

---

## 💡 **Problem-Solving Checklist**

Before coding:

- [ ] **Understand constraints:** Array size? Value range? Sorted?
- [ ] **Identify pattern:** Which pattern fits?
- [ ] **Choose data structure:** Vec? HashMap? HashSet?
- [ ] **Plan time/space:** Can I do better than O(n²)?
- [ ] **Handle edge cases:** Empty array? Single element? All same?

While coding:

- [ ] **Check bounds:** Every `nums[i]` access
- [ ] **Mutability:** Do I need `&mut`?
- [ ] **Ownership:** Am I consuming or borrowing?
- [ ] **Test mentally:** Walk through with example

---

## 🔑 **Key Takeaways**

1. **HashMap = O(1) lookup magic** (but uses O(n) space)
2. **Two Pointers = O(1) space win** (especially on sorted arrays)
3. **Sliding Window = Optimal for subarrays**
4. **Prefix Sum = Precalculate for range queries**
5. **Binary Search = Only works on sorted/monotonic data**
6. **When stuck → Start with brute force, optimize with patterns**

---
