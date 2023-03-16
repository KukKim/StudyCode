
// Test var

var i = 0

function functionA () {
    console.log(i)
    i = 1
    var j = 3
}

{
    var k = 5;
}

functionA(i)
console.log(i)
// console.log(j) // ReferenceError: j is not defined
console.log(k)

var a = "This is A"
console.log(a)
var a = "This is a"
console.log(a)
a = "Aa"
console.log(a)

console.log(hoistingTest)
var hoistingTest = "Hello hoisting!"