
// Test let

let i = 0

function functionA () {
    console.log(i)
    i = 1
    let j = 3
}

{
    let k = 5;
}

functionA(i)
console.log(i)
// console.log(j) // ReferenceError: j is not defined
// console.log(k) // ReferenceError: k is not defined

let a = "This is A"
console.log(a)
// let a = "This is a" // SyntaxError: Identifier 'a' has already been declared
// console.log(a)

{
    let a = "This is a"
    console.log(a)
}

a = "Aa"
console.log(a)


console.log(hoistingTest)
let hoistingTest = "Hello hoisting!"