/**
 * Events
 */
// function exampleEvent() {
//     var fs = require('fs');
//     var rs = fs.createReadStream('./demofile.txt');
//     rs.on('open', function () {
//         console.log('The file is open');
//     });
// }


// function customEvents() {
//     var events = require('events');
//     var eventEmitter = new events.EventEmitter();

//     //Create an event handler:
//     var myEventHandler = function () {
//         console.log('I hear a scream!');
//     }

//     //Assign the event handler to an event:
//     eventEmitter.on('scream', myEventHandler);

//     //Fire the 'scream' event:
//     eventEmitter.emit('scream');
// }

/**
 * Streams
 */

// function readableStreams() {
//     const Stream = require('stream');

//     const readableStream = new Stream.Readable({
//         read() {
//             console.log("Read a stream ...")
//         },
//     });
//     readableStream.push('hi!');
//     readableStream.push('ho!');
// }
// readableStreams();


// function writableStream() {
//     const Stream = require('stream');

//     const readableStream = new Stream.Readable({
//         read() { },
//     });
//     const writableStream = new Stream.Writable();

//     writableStream._write = (chunk, encoding, next) => {
//         console.log(chunk.toString());
//         next();
//     };

//     readableStream.pipe(writableStream);

//     readableStream.push('hi!');
//     readableStream.push('ho!');
// }

// writableStream();



// todo transform stream and duplex stream.