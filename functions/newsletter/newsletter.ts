import { Handler } from '@netlify/functions'
import fetch from 'node-fetch'
import 'dotenv/config'

export const handler: Handler = async (event, context) => {
  const { name = 'stranger' } = event.queryStringParameters
  const  BUTTONDOWN_API_KEY = process.env.BUTTONDOWN_API_KEY

  return {
    statusCode: 200,
    body: JSON.stringify({
      message: `Hello, ${name}! My key is ${BUTTONDOWN_API_KEY}`,
    }),
  }
}
