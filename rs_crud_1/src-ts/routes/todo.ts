import express from 'express';
import LibLoad from "../../LibLoad"

const router = express.Router();

router.post('/create', async function(req, res) {
  const retObj = {ret: 500, data: null};
  try {
    const lib = LibLoad.getLib();
    const todoAdd = lib.func('char* todo_add(const char* input)');    
    const body = req.body
    console.log(body);
    const j1 = JSON.stringify(body)
    todoAdd(j1);
    return res.json(body);
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
    const todo_delete = lib.func(
        "todo_delete",
        "int",
        ["int"]
    );   
    const body = req.body
    console.log(body);
    const resp = todo_delete(body.id);
    return res.json(body);
  } catch (error) {
    console.error(error);
    res.sendStatus(500);
  }
});

export default router;
