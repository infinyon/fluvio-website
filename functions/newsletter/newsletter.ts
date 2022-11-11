import { Handler } from '@netlify/functions'
import fetch from 'node-fetch'
import querystring from 'querystring';

const BUTTONDOWN_API_KEY = process.env.BUTTONDOWN_API_KEY
const BUTTONDOWN_API_URL = "https://api.buttondown.email/v1/subscribers"

interface User {
  name: string;
  email: string;
}

function isArray(obj: Object) {
  return Object.prototype.toString.call(obj) === '[object Array]';
}

export const handler: Handler = async (event, context) => {
  if (event.httpMethod !== "POST") {
    return { statusCode: 405, body: "Method Not Allowed" };
  }

  try {
    const user:User = JSON.parse(JSON.stringify(querystring.parse(event.body)));
    console.log(user);

    let response = await fetch(BUTTONDOWN_API_URL, {
      method: "POST",
      headers: {
        Authorization: `Token ${BUTTONDOWN_API_KEY}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email: user.email, notes: user.name }),
    });

    let message = await response.json();
    if (isArray(message)) {
      message = message[0];
    }

    console.log(`Buttondown Response (${response.status}): ${JSON.stringify(message)}`)
    return {statusCode: response.status, body: JSON.stringify(message)};

  } catch (err) {

    console.log(`Error: ${err.toString()}`);
    return { statusCode: 500, body: err.toString()};

  }
}
