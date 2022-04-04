import { Handler } from '@netlify/functions'
import fetch from 'node-fetch'
import querystring from 'querystring';

const  BUTTONDOWN_API_KEY = process.env.BUTTONDOWN_API_KEY
interface User {
  name: string;
  email: number;
  message: string;
}

export const handler: Handler = async (event, context) => {
  if (event.httpMethod !== "POST") {
    return { statusCode: 405, body: "Method Not Allowed" };
  }
  
  try {
    const user:User = JSON.parse(JSON.stringify(querystring.parse(event.body)));

    console.log(user);
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
