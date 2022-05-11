




/**
// array methods
//pop
const fruits = ["Banana", "Orange", "Apple", "Mango"];
// console.log("Fruits before pop " + fruits);
fruits.pop();
// console.log("Fruits after pop " + fruits);


// push 
console.log("Fruits before push " + fruits);
fruits.push("Kiwi");
console.log("Fruits after push " + fruits);

//delete
console.log("Fruits before delete " + fruits);
delete fruits[0];
console.log("Fruits after delete " + fruits);



// merge array
const arr1 = ["Cecilie", "Lone"];
const arr2 = ["Emil", "Tobias", "Linus"];
const arr3 = ["Robin", "Morgan"];

const myChildren = arr1.concat(arr2, arr3);
// use spread operator
console.log("Name after merge " + myChildren);

const myChildren2 = [...arr1, ...arr2, ...arr3];
console.log("Name after merge 2 " + myChildren);


const numbers = [45, 4, 9, 16, 25];
let txt = "";
numbers.forEach((e) => {
    console.log("at element" + e);
});


 * sets
 */
// Create a Set
const letters = new Set(["a", "b", "c"]);

// Add Values to the Set
// console.table( letters);
letters.add("d");
// console.table( letters);
/**** 
 * maps
 */
// Create a Map
const fruitsMap = new Map([
    ["apples", 500],
    ["bananas", 300],
    ["oranges", 200]
]);
// console.table("fruitsMap before set" + fruitsMap);
fruitsMap.set("apples", 200);
// console.table("fruitsMap after set" + fruitsMap);

//specific
console.log("Get a specific element" + fruitsMap.get("apples"));





// loops
// arrays
// functions 
// accessing dom
function carsList() {
    const cars = ["Subaru", "Volvo", "BMW"];
    const uiListElement = document.getElementById("car_types");
    for (let carItem = 0; carItem < cars.length; carItem++) {
        const element = cars[carItem];
        const node = document.createElement("li");
        const textnode = document.createTextNode(element);
        node.appendChild(textnode);
        uiListElement.appendChild(node);
    }
}
carsList();





function loopsMap() {
    // Create a Map
    const fruits = new Map([
        ["apples", 500],
        ["bananas", 300],
        ["oranges", 200]
    ]);
   // console.table(fruits);
    fruits.forEach(function (value, key) {
        console.log('key ' + key + '  value ' + value);
    });

}
loopsMap();