import express from 'express';
import cookieParser from "cookie-parser";

import todoRouter from './src-ts/routes/todo';
import indexRouter from './src-ts/routes/index';
//pages
import TodoMain from './src-ts/pages/TodoMain';

const app = express();
import 'dotenv/config'

app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(cookieParser());
app.use(express.static('public'));

const errorObj = {ret: "NG", messase: "Error"};

app.use('/api/todo', todoRouter);
app.use('/api', indexRouter);

//SSR
app.get('/', (req: any, res: any) => {
  try {
    res.send(TodoMain());
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
