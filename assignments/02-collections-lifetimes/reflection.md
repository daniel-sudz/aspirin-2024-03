# Reflection

I think for `longest_equal_sequence` and `is_valid_paranthesis` the functional approach with `fold` is very clean and much more readable compared to the prescriptive approach. One of the downsides though is that there's isn't a clean way to break/return from a `fold` operation apart from propagating an "error" state through which is what I did. This means that the prescriptive approach for `is_valid_paranthesis` will be faster since it can exit early when the answer is false. 

Other than that, implementing a functional approach is much harder when you need random access to an array. For instance in the `longest_common_substring` problem I used a 2D DP approach that would be very hard to implement functionally in a performant way. 