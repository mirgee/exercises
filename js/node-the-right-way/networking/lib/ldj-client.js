const EventEmitter = require('events').EventEmitter;

class LDJClient extends EventEmitter {
  constructor(stream) {
    if (!stream) {
      throw Error('Error: No stream specified.');
    }
    super();
    let buffer = '';
    stream.on('data', data => {
      buffer += data;
      let boundary = buffer.indexOf('\n');
      while (boundary !== -1) {
        const input = buffer.substring(0, boundary);
        buffer = buffer.substring(boundary + 1);
        boundary = buffer.indexOf('\n');
        try {
          this.emit('message', JSON.parse(input));
        } catch (err) {
          console.error(`Failed to process message '${input}'`);
        }
      }
    });

    stream.on('close', () => {
      let boundary = buffer.indexOf('\n');
      buffer = buffer.substring(boundary+1); 
      this.emit('message', JSON.parse(buffer));
    }); 

    // TODO: Maybe emit a close event when the file is deleted?
  }

  static connect(stream) {
    return new LDJClient(stream);
  }
}

module.exports = LDJClient;
