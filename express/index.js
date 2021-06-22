const fs = require('fs');
const express = require('express');
const app = express();

app.get('/', (req, res) => {
  res.status(200).header("Content-Type", "video/MP2T");
  fs.createReadStream(process.argv[2]).pipe(res);
});

app.listen(8080)
