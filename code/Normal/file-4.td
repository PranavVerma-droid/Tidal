/* Comments Are Added, Using this Syntax */
/* This will Also be Ignored. */

/* var a = 10; 
   var b = 100;
   novar c = 100; 
   
   All of these Code Snippets Will be Ignored.*/

novar a = "hello";
novar b = ("world");

print(b[0] * 10);
print(a[1] * 10);
print("");
print((a + " " /*+ b*/ + "  ") * 3);

/* Here, b will be ignored, so, it will not print. */
