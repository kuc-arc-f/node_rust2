import express from 'express';
import LibConfig from "../lib/LibConfig";
import LibLoad from "../../LibLoad"

const router = express.Router();
const COOKIE_NAME = LibConfig.COOKIE_NAME;

router.post('/login', async function(req, res) {
  const retObj = {ret: 500, data: null};
  try {
    const lib = LibLoad.getLib();
    const user_login = lib.func('int user_login(const char* input)'); 

    const body = req.body
    console.log(body);
    const j1 = JSON.stringify(body)
    const resp = user_login(j1);
    console.log("resp=", resp);
    let htm = `<div class="font-bold text-2xl bg-red-400 text-white p-2">Error</div>`;
    if (resp > 0){
      //生存期間( msec ) Nday
      res.cookie(COOKIE_NAME , "1", {
        maxAge: 30 * 24 * 60 *  60 * 1000,
        httpOnly: false
      })      
      htm = `<div><input type="text" id="result_login" value="1" /></div>`;
      return res.send(htm)
    }
    return res.send(htm)
  } catch (error) {
    console.error(error);
    res.sendStatus(500);
  }
});

router.get('/list', async function(req, res) {
  const retObj = {ret: 500, data: null};
  try {
    const lib = LibLoad.getLib();
    const todo_list = lib.func('char* todo_list()');    
    const body = req.body
    const resp = todo_list();
    return res.send(resp)
  } catch (error) {
    console.error(error);
    res.sendStatus(500);
  }
});

router.get('/get/:id', async function(req, res) {
  const retObj = {ret: 500, data: null};
  try {
    const lib = LibLoad.getLib();
    const id = req.params.id;
    console.log("id=", id)
    const todo_get = lib.func(
        "todo_get",
        "char*",
        ["int"]
    );    
    const resp = todo_get(Number(id));
    return res.send(resp);
  } catch (error) {
    console.error(error);
    res.sendStatus(500);
  }
});

router.post('/delete', async function(req, res) {
  const retObj = {ret: 500, data: null};
  try {
    const lib = LibLoad.getLib();
    const todo_list = lib.func('char* todo_list()');   
    const todo_delete = lib.func(
        "todo_delete",
        "int",
        ["int"]
    );   
    const body = req.body
    console.log(body);
    todo_delete(Number(body.id));
    const resp = todo_list();
    return res.send(resp)
  } catch (error) {
    console.error(error);
    res.sendStatus(500);
  }
});

export default router;
