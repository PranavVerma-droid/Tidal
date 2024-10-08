var previousVariable = 1.0;
var currentVariable = 1.0;
var nextVariable;

var finalResult = "0 1 1 ";

for(var i = 0; i >= 0; i = i + 1;) {
    nextVariable = float(previousVariable + currentVariable);
    finalResult = (finalResult + str(nextVariable) + " ");
    previousVariable = currentVariable;
    currentVariable = nextVariable;
}

print(finalResult);




/* print(0);
print(1);
print(1);

for(var i = 0; i <= 1500; i = i + 1;) {
    nextVariable = float(previousVariable + currentVariable);
    print(nextVariable);
    previousVariable = currentVariable;
    currentVariable = nextVariable;
} */
