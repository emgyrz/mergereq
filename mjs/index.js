
'use strict';

var VERSION = require('./package.json').version;

var path = require('path');

module.exports =
  process.platform === 'darwin'
    ? path.join(__dirname, 'mergereq-osx64', 'mergereq') :
    process.platform === 'linux' && process.arch === 'x64'
      ? path.join(__dirname, 'mergereq-linux64', 'mergereq') :
      process.platform === 'win32' && process.arch === 'x64'
        ? path.join(__dirname, 'mergereq-win64', 'mergereq.exe') :
        null;
