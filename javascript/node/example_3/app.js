function readAsync() {
    console.log("==========readAsync ===========");
    const fs = require('fs');
    const path = require("path");
    const file = path.join(__dirname, "sample.txt")
    fs.readFile(file, 'utf8', (err, data) => {
        if (err) {
            console.error(err);
            return;
        }
        console.log(data);
    });
}

readAsync()

// function readSync() {
    // console.log("==========readAsync ===========");
//     const fs = require('fs');
//     const path = require("path");
//     const file = path.join(__dirname, "sample.txt")
//     try {
//         const data = fs.readFileSync(file, 'utf8');
//         console.log(data);
//     } catch (err) {
//         console.error(err);
//     }

// }
// readSync();


// const fs = require('fs/promises');

// async function example() {
    // console.log("==========readAsync ===========");
//     const path = require("path");
//     const file = path.join(__dirname, "sample.txt")
//     try {
//         const data = await fs.readFile(file, { encoding: 'utf8' });
//         console.log(data);
//     } catch (err) {
//         console.log(err);
//     }
// }
// example();