// console.log("Hello")

// setTimeout(() => {
//     console.log("3 seconds")
// }, 3000),

// console.log("Good bye!")


// const promise = new Promise((resolve, reject) => {
//     if (true) {
//       resolve('resolved');
//     } else {
//       reject('rejected');
//     }
// })

// const fruits = new Map([
//   ["apples", 500],
//   ["bananas", 300],
//   ["oranges", 200]
//   ]);

// console.log(fruits.get("apples"))



// var testObj = {
//   testKey1 : "aaa",
//   testKey2 : "111",
//   set setKey1(value) {
//     this.testKey1 = value
//   },
//   set setKey2(value) {
//     this.testKey2 = value
//   }
// }

// testObj.setKey1 = "bbb"
// console.log(testObj)



var setTimeoutFunc = () => {
  setTimeout(() => {
    console.log("time out done")
  }, 0)
}

const promise = new Promise((resolve, reject) => {
  console.log("promise")
  resolve()
})


setTimeoutFunc()

promise.then(() => {
  console.log("promise done")
})
.then(() => {
  console.log("two")
})