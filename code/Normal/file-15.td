var a = [12, 23, "hi", "hello", "hi", "hi"];
a[0] = 10;
novar b = [12, 34123, ["hi", [[["hi"]], "lol", [[[[[[[0, 1, 2 ,3]]]]]], 4, 5, 6]]]];
var c = [];

/* Try Uncommenting This Line: */
/* a[6] = 10; */

print(a);
print(b);
print(c);

print("");


print(a[1]);
print(a[2][1]);
print(b[2][0][1]);


print("");

print(type(a));
print(type(a));
print(type(b));

print("");

print(type(a[0]));
print(type(a[0]));
print(type(b[2][0]));
print(type(b[2][1]));
