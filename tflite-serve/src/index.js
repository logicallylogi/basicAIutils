const express = require('express')
const ai = require('./lib.js')
const app = express()
const port = 3000

app.get('/ai', (req, res) => {
  res.send(ai.predict(req.body.msg))
})

app.listen(port, () => {
  console.log(`TFLite Server listening at http://localhost:${port}`)
})