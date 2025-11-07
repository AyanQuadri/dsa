# 🦀 Rust Arrays/Vectors

Quick reference for solving array problems in Rust.

---

## 📦 **1. Creating Arrays/Vectors**

### **Vec (Dynamic - Most Common)**

```rust
// Empty vector
let mut nums: Vec<i32> = Vec::new();
let mut nums = vec![];

// With initial values
let nums = vec![1, 2, 3, 4, 5];

// With capacity (performance)
let mut nums = Vec::with_capacity(10);

// Repeat value
let nums = vec![0; 5];  // [0, 0, 0, 0, 0]

// From range
let nums: Vec<i32> = (1..=5).collect();  // [1, 2, 3, 4, 5]
```

### **Arrays (Fixed Size)**

```rust
let nums = [1, 2, 3, 4, 5];
let nums: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 100];  // 100 zeros
```

---

## 🔍 **2. Accessing Elements**

```rust
let nums = vec![10, 20, 30, 40];

// Indexing (panics if out of bounds)
let first = nums[0];           // 10
let last = nums[nums.len() - 1];  // 40

// Safe access (returns Option)
let first = nums.get(0);       // Some(&10)
let invalid = nums.get(10);    // None

// First/Last
let first = nums.first();      // Some(&10)
let last = nums.last();        // Some(&40)

// Pattern matching
if let Some(&val) = nums.get(2) {
    println!("{}", val);
}
```

---

## 🔄 **3. Iterating**

```rust
let nums = vec![1, 2, 3, 4, 5];

// Immutable iteration
for num in &nums {
    println!("{}", num);
}

// Mutable iteration
for num in &mut nums {
    *num *= 2;
}

// With index
for (i, num) in nums.iter().enumerate() {
    println!("nums[{}] = {}", i, num);
}

// Consuming (takes ownership)
for num in nums {
    println!("{}", num);
}
// nums is no longer usable here
```

---

## ✏️ **4. Modifying Vectors**

```rust
let mut nums = vec![1, 2, 3];

// Add elements
nums.push(4);              // [1, 2, 3, 4]
nums.extend([5, 6]);       // [1, 2, 3, 4, 5, 6]
nums.insert(0, 0);         // [0, 1, 2, 3, 4, 5, 6]

// Remove elements
nums.pop();                // [0, 1, 2, 3, 4, 5]
nums.remove(0);            // [1, 2, 3, 4, 5]

// Modify by index
nums[0] = 10;              // [10, 2, 3, 4, 5]

// Clear all
nums.clear();              // []
```

---

## 🔀 **5. Slicing**

```rust
let nums = vec![1, 2, 3, 4, 5];

// Slice (returns &[T])
let slice = &nums[1..4];   // [2, 3, 4]
let slice = &nums[..3];    // [1, 2, 3]
let slice = &nums[2..];    // [3, 4, 5]
let slice = &nums[..];     // [1, 2, 3, 4, 5]

// Mutable slice
let mut nums = vec![1, 2, 3];
let slice = &mut nums[1..];
slice[0] = 20;             // nums is now [1, 20, 3]
```

---

## 🎯 **6. Passing to Functions**

### **Borrow (Read-Only)**

```rust
fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

let nums = vec![1, 2, 3];
let total = sum(&nums);    // nums still usable
```

### **Mutable Borrow**

```rust
fn double(nums: &mut [i32]) {
    for num in nums {
        *num *= 2;
    }
}

let mut nums = vec![1, 2, 3];
double(&mut nums);         // nums is now [2, 4, 6]
```

### **Take Ownership**

```rust
fn consume(nums: Vec<i32>) -> i32 {
    nums.len() as i32
}

let nums = vec![1, 2, 3];
let len = consume(nums);
// nums is no longer usable
```

### **Return New Vector**

```rust
fn clone_and_modify(nums: &[i32]) -> Vec<i32> {
    nums.iter().map(|x| x * 2).collect()
}

let nums = vec![1, 2, 3];
let doubled = clone_and_modify(&nums);
```

---

## 🧬 **7. Cloning & Copying**

```rust
let nums = vec![1, 2, 3];

// Clone (deep copy)
let copy = nums.clone();

// Convert to array slice
let slice: &[i32] = &nums;

// Copy from slice to vec
let new_vec = slice.to_vec();
```

---

## 🔧 **8. Common Patterns**

### **Two Pointers**

```rust
let mut left = 0;
let mut right = nums.len() - 1;

while left < right {
    // Process
    left += 1;
    right -= 1;
}
```

### **Sliding Window**

```rust
let mut sum = 0;
for i in 0..k {
    sum += nums[i];
}

for i in k..nums.len() {
    sum += nums[i] - nums[i - k];
}
```

### **Reverse**

```rust
nums.reverse();

// Or manually
let mut i = 0;
let mut j = nums.len() - 1;
while i < j {
    nums.swap(i, j);
    i += 1;
    j -= 1;
}
```

### **Sort**

```rust
nums.sort();                    // ascending
nums.sort_by(|a, b| b.cmp(a));  // descending
```

### **Filter/Map/Collect**

```rust
let evens: Vec<i32> = nums.iter()
    .filter(|&x| x % 2 == 0)
    .cloned()
    .collect();

let doubled: Vec<i32> = nums.iter()
    .map(|x| x * 2)
    .collect();
```

### **Find Max/Min**

```rust
let max = *nums.iter().max().unwrap();
let min = *nums.iter().min().unwrap();
```

---

## ⚠️ **9. Common Pitfalls**

### **Borrowing Errors**

```rust
// ❌ Can't borrow as mutable while immutable borrow exists
let nums = vec![1, 2, 3];
let first = &nums[0];
nums.push(4);  // ERROR

// ✅ Drop immutable borrow first
let nums = vec![1, 2, 3];
{
    let first = &nums[0];
}
nums.push(4);  // OK
```

### **Index Out of Bounds**

```rust
// ❌ Panics
let val = nums[100];

// ✅ Safe access
if let Some(&val) = nums.get(100) {
    // Handle
}
```

### **Moving Out of Vec**

```rust
// ❌ Can't move out of indexed content
let val = nums[0];  // ERROR if T doesn't implement Copy

// ✅ Use reference or clone
let val = &nums[0];
let val = nums[0].clone();
```

---

## 🎯 **10. Patterns**

### **Return Vec from Function**

```rust
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    vec![0, 1]
}
```

### **Use Slice Instead of Vec**

```rust
pub fn two_sum(nums: &[i32], target: i32) -> Vec<i32> {
    // More flexible - accepts both &Vec and &[]
}
```

### **Convert String to Vec**

```rust
let s = "hello";
let chars: Vec<char> = s.chars().collect();
```

### **HashMap with Vec**

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
for (i, &num) in nums.iter().enumerate() {
    if let Some(&j) = map.get(&(target - num)) {
        return vec![j, i];
    }
    map.insert(num, i);
}
```

---

## 📚 **Quick Reference**

| Operation | Vec              | Array            |
| --------- | ---------------- | ---------------- |
| Create    | `vec![1,2,3]`    | `[1,2,3]`        |
| Access    | `nums[i]`        | `nums[i]`        |
| Length    | `nums.len()`     | `nums.len()`     |
| Push      | `nums.push(x)`   | ❌               |
| Pop       | `nums.pop()`     | ❌               |
| Iterate   | `for n in &nums` | `for n in &nums` |
| Slice     | `&nums[..]`      | `&nums[..]`      |
| Clone     | `nums.clone()`   | `nums.clone()`   |

---
