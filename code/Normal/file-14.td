var x = 0;
var y = 5;

if (x > 0 && y < 10) {
    print("x is positive and y is less than 10");
} elif (x <= 0 || y >= 10) {
    print("x is non-positive or y is greater than or equal to 10");
} else {
    print("This should never happen");
}

x = true;
print(!x);
print(!(x && false)); 