import { Handler } from '@netlify/functions'
import fetch from 'node-fetch'

if (!process.env.NETLIFY) {
    import 'dotenv/config'
   // get local env vars if not in Netlify
}

const  BUTTONDOWN_API_KEY = process.env.BUTTONDOWN_API_KEY
interface User {
  name: string;
  email: number;
  botField: string;
}

export const handler: Handler = async (event, context) => {
  try {
    const data = JSON.parse(event.body).payload.data;
    console.log(data);
    return {
        statusCode: 200,
        body: JSON.stringify({
            message: `POST Success`
        })
    };
  } catch (err) {
    console.error('error ocurred in processing ', event);
    console.error(err);
    return {
      statusCode: 500,
      body: err.toString()
    };
  }
}
