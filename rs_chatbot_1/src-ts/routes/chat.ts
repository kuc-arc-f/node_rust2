import express from 'express';
import LibLoad from "../../LibLoad"

const router = express.Router();

router.post('/send', async function(req, res) {
  const retObj = {ret: 500, data: null};
  try {
    const lib = LibLoad.getLib();
    const body = req.body
    console.log(body);
    const chat_send = lib.func(
        "chat_send",
        "char*",
        ["char*"]
    );        
    const resp = chat_send(body.query);      
    return res.send(resp)
  } catch (error) {
    console.error(error);
    res.sendStatus(500);
  }
});

export default router;
