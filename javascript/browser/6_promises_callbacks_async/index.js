// let myFunction = function (myResolve, myReject) {
//     //network or file operation
//     setTimeout(
//         function () { myResolve("I love You !!"); }, 3000);
// }

// let myPromise = new Promise(myFunction);

// myPromise.then(function (value) {
//     document.getElementById("demo").innerHTML = value;
// }).catch((e)=>{
//     console.error("Errr "+ e)
// });


/**
 * second example
 */
// using call backs
function getFile(myCallback) {
    let req = new XMLHttpRequest();
    req.open('GET', "www.google.com");
    req.onload = function () {
        if (req.status == 200) {
            myCallback(req.responseText);
        } else {
            myCallback("Error: " + req.status);
        }
    }
    req.send();
}
getFile((myPar) => { console.log(myPar) });

let myPromise2 = new Promise(function (myResolve, myReject) {
    let req = new XMLHttpRequest();
    req.open('GET', "www.google.com");
    req.onload = function () {
        if (req.status == 200) {
            myResolve(req.response);
        } else {
            myReject("File not Found");
        }
    };
    req.send();
});

myPromise2.then(
    function (value) { myDisplayer(value); },
    function (error) { myDisplayer(error); }
);



/*
// async  await 
async function myDisplay() {
    let myPromise = new Promise(function (resolve, reject) {
        resolve("I love You !!");
    });
    document.getElementById("demo").innerHTML = await myPromise;
}

myDisplay();
**/



//async await

function resolveAfter2Seconds() {
    return new Promise(resolve => {
        setTimeout(() => {
            resolve('resolved');
        }, 2000);
    });
}

async function asyncCall() {
    console.log('calling');
    const result = await resolveAfter2Seconds();
    console.log(result);
    // expected output: "resolved"
}

asyncCall();
