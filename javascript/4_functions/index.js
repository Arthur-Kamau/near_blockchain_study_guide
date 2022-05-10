


//basic function
function myFunction() {
   console.log("I am here in function")
  }

//return and par
function myFunction(p1, p2) {
    return p1 * p2;   // The function returns the product of p1 and p2
  }


const resposnes = myFunction(300*400);
console.log(" result is "+resposnes)




//callback / predicate
setTimeout(function () {
    const date = new Date();
    console.log("here" + document.getElementsByTagName("h6"));
    document.getElementsByTagName("h6")[0].innerHTML = "Cars loaded, time is " + date.toLocaleTimeString();
}, 1000);