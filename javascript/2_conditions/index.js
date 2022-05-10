



if(4 === 4.0){
    window.alert("they are the same")
}


if (5=="5"){
    window.alert("they are the same")
}

// one is true
const names = window.prompt("input your name ");
if(names == null ||  names == undefined ){
    window.alert("no name found");
}else if (names.length == 0 ){
    window.alert("name is empty");
}else{
    window.alert(`name is ${names}`)
}


// both are true
const randNumber = window.prompt("input a random number ");
let randInt = parseInt(randInt);
if(randInt >10 && randInt < 100  ){
    window.alert("the number is between 10 and 100");
}else{
    window.alert(`the number is unknow`)
}
