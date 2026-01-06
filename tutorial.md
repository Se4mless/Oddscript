# Oddscript Tutorial

### Containers:
- Containers are your variables, in total, containers allow you to store 256 8 bit numbers or 65536 total bits
### Container Selection:
- To Select The Currently Edited Container, You Use The > Symbol, Here is how it would look in practice setting the current container to be the 24th: `> 24`
### Container Manipulation
- There Are Many Ways To Manipulate A Container, The Most Simple is with the `=` symbol. This Symbol Sets The Current Containers value to be the value on the right i.e. `= 7` would set the container to be 7
<br>
- Another Method Of Manipulation is the arithmetic operations, these operations allow you to manipulate the current containers value arithmetically from another containers index. Here is it in practice:
```
# set the 7th container to be 12
> 7
= 12 

# Set the 2nd container to be 24
> 2
= 24
# Adds The value of the 7th container to the 2nd container
+ 7

```
<br>
- Finally, You Can Use the ~ symbol to set the value to a user input i.e.
```
> 1
# Sets The Value of 1 to be the user input
~ 
```

### Output
- To Ouput the value of the current container, All you have to do is use the `<` symbol, here it is in practice :
```
> 1
= 21
<
# Would Output 21
```
### Comments:.
- Comments are declared by using a `#` symbol at the start of the line.

### Loops:
- Loops are a section of code that repeats until the current container reaches the value of another
- Loops Are Declared by usign the `?` symbol followed by the container to compare with, a loops end point is provided by the usage of the `.` symbol.
- An example of how a loop may look is shown below :
```
# The Increment Value
> 1
= 1

# Repeats 3 times
> 2
= 3


> 3
= 0
# Check if current container = the value in container 2
? 2

# Output The Value
<
# Increments the current container by the value in container 1
+ 1
# End Of Loop
.
```
Output:
```
0 1 2
```
- While this language does not have if statements, you can use the loop to achieve a similar result shown in this example :
```
# The first value
> 1
= 2

# The second value
> 2
= 4

# Temporary Value Set To 2's value
> 3
= 0
+ 2

# Loop Check
? 1
# Run Inner Code (i.e. Output the value)
<
# Set the container = to the first value so it does not run again
= 0 
+ 1
.
```

