const element = [1,2,3,4,5]
for (let index = 0; index < array.length; index++) {
    const element = array[index];
    console.log("At number "+ element)
}

const letters = ["a", "b", "c"];
for (const x of letters) {
    // code block to be executed
    console.log("at letter "+x)
}


const city = ["Nairobi", "Kisumu", "Mombasa"];
city.forEach(element => {
    console.log("at city "+ element)
});



// less than 20
let increment =0
let max = 20
while (increment<max) {
  console.log(`increment is less than ${max} increment val is ${increment} `)
}