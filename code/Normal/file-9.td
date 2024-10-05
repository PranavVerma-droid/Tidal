var a = 10.23;
var b = 123;


print(a / b);

a = 0.0;

if (type(a) == float) {
    print("a is a float");
} else {
    print("a is not a float.");
}

for(var i = 0; i < 10; i = i + 1;) {
    print(i);
    if (i == 5) {
        break;
    } else {
        a = a + i;
    }
} 