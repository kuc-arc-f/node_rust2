
import express from 'express';
import cookieParser from "cookie-parser";

import chatRouter from './src-ts/routes/chat';
import indexRouter from './src-ts/routes/index';
//pages
import ChatMain from './src-ts/pages/ChatMain';
import SsrAbout from './src-ts/pages/About';

const app = express();
import 'dotenv/config'

app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(cookieParser());
app.use(express.static('public'));

const errorObj = {ret: "NG", messase: "Error"};

///api/todos
app.use('/api/chat', chatRouter);
app.use('/api', indexRouter);

app.get('/about', (req: any, res: any) => {
  try {
    const htm = SsrAbout();
    res.send(htm);
  } catch (error) {
    res.sendStatus(500);
  }
});

app.get('/', (req: any, res: any) => {
  try {
    const topStr = ChatMain();
    res.send(topStr);
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
