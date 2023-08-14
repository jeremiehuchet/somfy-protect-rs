const express = require('express')
const enableExpressWs = require('express-ws')
const bodyParser = require('body-parser')
const fs = require('fs')

const app = express()
const port = 3000
const global = {
  exchangePasswordCount: 0,
  refreshTokenCount: 0,
  wsMessagesReceived: [],
  wsMessagesToSend: []
}

enableExpressWs(app);

app.use(bodyParser.urlencoded({ extended: false }))
app.use(bodyParser.json())

app.use((req, res, next) => {
  console.info(`${Date.now()}: ${req.method} ${req.url}`)
  if (req.path.indexOf('/api') === 0) {
    const authozization = req.header('Authorization')
    if (!authozization || authozization.indexOf('Bearer valid') !== 0) {
      res.body = {
        "data": [],
        "message": "error.unauthorized",
        "uid": "649b6ba5eeb9f"
      }
      res.sendStatus(401)
      return
    }
  }
  next()
})

app.get('/mock/exchange-password-count', (req, res) => {
  res.send(`${global.exchangePasswordCount}`)
})

app.get('/mock/refresh-token-count', (req, res) => {
  res.send(`${global.refreshTokenCount}`)
})

app.get('/mock/ws-messages-received', (req, res) => {
  res.send(JSON.stringify(global.wsMessagesReceived))
})

app.put('/mock/ws-messages-to-send', (req, res) => {
  global.wsMessagesToSend = req.body
  res.json(global.wsMessagesToSend)
})

app.post('/auth/token', (req, res) => {
  if (req.header('Authorization') !== 'Basic c29tZnk6c29tZnkrc2VjcmV0') {
    res.body = {
      error: 'unauthorized_client'
    }
    res.sendStatus(400)
  }
  if (req.body.grant_type === 'password') {
    global.exchangePasswordCount++
  }
  if (req.body.grant_type === 'refresh_token') {
    global.refreshTokenCount++
  }
  res.send({
    access_token: "valid_access_token",
    token_type: "Bearer",
    expires_in: 1,
    refresh_token: "valid_refresh_token"
  })
})

app.get('/api/site', (req, res) => {
  if (req.header('Authorization').indexOf('Bearer valid') != 0) {

  } else {
    const sites = fs.readFileSync('list_sites.json', 'utf8')
    res.send(sites)
  }
})

app.get('/api/site/:siteId/device', (req, res) => {
  if (req.params.siteId == 'Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT') {
    const devices = fs.readFileSync('list_devices.json', 'utf8')
    res.send(devices)
  } else {
    res.sendStatus(404)
  }
})

app.get('/api/site/:siteId/device-compatible', (req, res) => {
  if (req.params.siteId == 'Szr5IxqYraaPqh2FFGNms2BQUT0R0hNT') {
    const compatibleDevices = fs.readFileSync('list_compatible_devices.json', 'utf8')
    res.send(compatibleDevices)
  } else {
    res.sendStatus(404)
  }
})

app.ws('/websocket', (ws, req) => {

  ws.on('error', error => {
    console.info(`/websocket Error(${error})`)
    global.wsMessagesReceived.push(error)
  })

  ws.on('message', msg => {
    console.info(`/websocket Text(${msg})`)
    global.wsMessagesReceived.push(JSON.parse(msg))
  })

  ws.on('close', msg => {
    console.info(`/websocket Close(${msg}`)
    global.wsMessagesReceived.push(JSON.parse(msg))
  })

  if (req.query.token === 'valid-token') {
    console.info('/websocket Connect')

    ws.send(JSON.stringify({
      key: "websocket.connection.ready"
    }))
    global.wsMessagesToSend.forEach(msg => {
      console.log('sending mocked WS message: ', msg)
      ws.send(JSON.stringify(msg))
    })
  } else {
    ws.send('websocket.error.token')
  }
})

app.listen(port, () => {
  console.info(`Somfy Protect API mock listening on port ${port}`)
})
