# Exercise: Capturing age as a number in Rust Systems Programming
Consider the range of numbers these data types can hold and choose what would be the best fit.

# Solution:
Choosing the Right Numeric Data Type

Let's analyze the potential range of human ages:

Negative ages: While theoretically possible in certain contexts (like historical calculations), for a general user input scenario, negative ages are unlikely.

Maximum age: While there have been documented cases of people living to extreme ages, a reasonable upper limit for practical purposes might be around 150 years.

Given these considerations, we can eliminate signed integer types like i8, i16, and i32 as they can represent negative numbers.

Among the unsigned integer types:

u8**:** Can represent numbers from 0 to 255. This is sufficient for most ages but might be limiting in very specific cases.

u16**:** Can represent numbers from 0 to 65,535. This is more than enough to accommodate any realistic human age.

u32**:** Can represent much larger numbers (0 to 4,294,967,295). While it's overkill for age, it's still a valid choice.

Considering Memory Usage and Performance:

While u32 can technically hold the value, using a larger data type than necessary can lead to unnecessary memory usage and potential performance overhead in certain operations.

Conclusion:

For the given scenario, u16 is the most suitable data type. It provides a sufficient range of values to represent any human age, while being efficient in terms of memory usage and performance.
