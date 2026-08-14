
import express from 'express';
import cookieParser from "cookie-parser";

import LibConfig from './src-ts/lib/LibConfig';
import userRouter from './src-ts/routes/user';
import todoRouter from './src-ts/routes/todo';
import indexRouter from './src-ts/routes/index';
//pages
import Login from './src-ts/pages/Login';
import SsrAbout from './src-ts/pages/About';

const app = express();
import 'dotenv/config'

app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(cookieParser());
app.use(express.static('public'));

const errorObj = {ret: "NG", messase: "Error"};

app.use('/api/user', userRouter);
app.use('/api/todo', todoRouter);
app.use('/api', indexRouter);

//Middleware
app.get('/*', function(req, res, next) {
  const COOKIE_NAME = LibConfig.COOKIE_NAME;
  //console.log(req.cookies[ COOKIE_NAME ]);
  if (req.path !== "/login") {
    if (!req.cookies[ COOKIE_NAME ]) {
      return res.redirect('/login');
    }
  }
  next();
});
app.get('/about', (req: any, res: any) => {
  try {
    const htm = SsrAbout();
    res.send(htm);
  } catch (error) {
    res.sendStatus(500);
  }
});

app.get('/login', (req: any, res: any) => {
  try {
    const topStr = Login();
    res.send(topStr);
  } catch (error) {
    res.sendStatus(500);
  }
});
app.get('/', (req: any, res: any) => {
  try {
    res.send("home");
  } catch (error) {
    res.sendStatus(500);
  }
});

//start
const PORT = 3000;
app.listen({ port: PORT }, () => {
  console.log(`Start-Server: http://localhost:${PORT}`);
});
console.log('start');
