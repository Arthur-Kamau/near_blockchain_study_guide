//js loaded
// alert("hello..");


// loops
// arrays
// functions 
// accessing dom
function carsList() {
    const cars = ["Saab", "Volvo", "BMW"];
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
    console.table(fruits);
    fruits.forEach(function (value, key) {
        console.log('key ' + key + '  value ' + value);
    })

}
loopsMap();

//callback
setTimeout(function () {
    const date = new Date();
    console.log("here" + document.getElementsByTagName("h6"));
    document.getElementsByTagName("h6")[0].innerHTML = "Cars loaded, time is " + date.toLocaleTimeString();
}, 1000);



// input handlers
const source = document.getElementById('source');
const result = document.getElementById('result');

const inputHandler = function (e) {
    result.innerHTML = e.target.value;
}

source.addEventListener('input', inputHandler);
source.addEventListener('propertychange', inputHandler); // for IE8



// objects
let person = { firstName: "John", lastName: "Doe", age: 50, eyeColor: "blue" };

console.error(person.firstName);

// delete person.age;
// console.error(person.age);



