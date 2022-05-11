import fetch from 'node-fetch';
console.log("Hello .......")

function appPromises() {

    console.log("==========appPromises ===========");
    fetch("http://example.com/api/endpoint")
        .then((response) => {
            // Do something with response
            console.log("Response " + JSON.stringify(response.body));
        })
        .catch(function (err) {
            console.error("Unable to fetch -", err);
        });
}


async function appAsync() {
    console.log("==========appAsync ===========");
    try {
        var response = await fetch("http://example.com/api/endpoint");
        console.log("Response " + JSON.stringify(response.body));
    } catch (error) {
        console.error("Unable to fetch -", err);
    }

}





appAsync();