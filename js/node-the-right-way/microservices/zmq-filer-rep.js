'use strict';
const fs = require('fs');
const zmq = require('zeromq');

// Socket to reply to client requests.
const responder = zmq.socket('rep');

// Handle incoming requests.
responder.on('message', data => {

	// Parse the incoming message.
	const request = JSON.parse(data);
	console.log(`Received request to get: ${request.path}`);

	// Read the file and reply with content.
	fs.readFile(request.path, (err, content) => {
    if (err) {
      console.error('Error in readFile:\n' + err);
    }
		console.log('Sending response content.');
    responder.send(JSON.stringify({
        content: content == null ? null : content.toString(),
        timestamp: Date.now(),
        pid: process.pid,
        err: err
    }));
  });
});

// Listen on TCP port 60401.
responder.bind('tcp://127.0.0.1:60401', err => {
	console.log('Listening for zmq requesters...');
});

// Close the responder when the Node process ends.
process.on('SIGINT', () => {
	console.log(`Shutting down...`);
	responder.close();
});

process.on('uncaughtException', (err) => {
	console.log(`Uncaught exception: \n${err}`);
});
