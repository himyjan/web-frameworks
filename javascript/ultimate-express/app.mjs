import express from 'ultimate-express';

const app = express();

app.set('etag', false);
app.set('x-powered-by', false);

app.get('/', function (req, res) {
  res.send('');
});

app.get('/user/:id', function (req, res) {
  res.send(req.params.id);
});

app.post('/user', function (req, res) {
  res.send('');
});

app.listen(3000);
