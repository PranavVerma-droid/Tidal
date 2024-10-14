var arr = [3, 1, 2];
print(arr);
arr.sort();
print(arr);
arr.reverse();
print(arr);
arr.insert(1, 4);
print(arr);
print(arr.count());
arr.clear();
print(arr);
print(arr.count());

var arr2 = arr.copy();

var str = " Hello World ";
print(str);
str = str.strip();
print(str);
str = str.upper();
print(str);
str = str.lower();
print(str);

var arr3 = ["heloo", "hi"];
var arr4 = ["Helo"];
arr3.insert(arr4);
arr3.insert([123, 123, 43]);
print(arr3);