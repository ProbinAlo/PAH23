var express = require('express');
var router = express.Router();

/* GET home page. */
router.get('/', function(req, res, next) {
  res.render('index', { title: 'ProbinAlo Hackathon 2023' });
});

module.exports = router;
