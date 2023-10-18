const express = require('express');

const app = express();

app.get('/', (req, res) => {
	setTimeout(() => {
		res.send('Hello World!');
	}, 60000);  // 60 seconds delay
});

app.listen(3000, () => {
	console.log('Server is running on port 3000');
});
