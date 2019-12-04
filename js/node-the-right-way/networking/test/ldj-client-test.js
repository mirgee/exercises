'use strict';
const assert = require('assert');
const EventEmitter = require('events').EventEmitter;
const LDJClient = require('../lib/ldj-client.js');

describe('LDJClient', () => {
  let stream = null;
  let client = null;

  beforeEach(() => {
    stream = new EventEmitter();
    client = new LDJClient(stream);
  });

  it('should emit a message event from a single data event', done => {
    client.on('message', message => {
      assert.deepEqual(message, {foo: 'bar'});
      done();
    });
    stream.emit('data', '{"foo":"bar"}\n');
  });

	it('should emit a message event from split data events', done => {
		client.on('message', message => {
			assert.deepEqual(message, {foo: 'bar'});
			done();
		});
		stream.emit('data', '{"foo":');
		process.nextTick(() => stream.emit('data', '"bar"}\n'));
	});

	it('should throw error on null stream', done => {
    assert.throws(() => {
      new LDJClient(null);
    }, Error, 'Error: No stream specified');
    done();
	});

	it('should process the rest of buffer on no newline and close', done => {
		client.on('message', message => {
			assert.deepEqual(message, {foo: 'bar'});
      done();
		});
    stream.emit('data', '{"foo":"bar"}');
    stream.emit('close');
	});

	it('should discard buffer when not valid json and process the next', done => {
		client.on('message', message => {
			assert.deepEqual(message, {foo: 'bar'});
      done();
		});
    stream.emit('data', 'some nonsense\n{"foo":"bar"}');
    done();
	});
});
