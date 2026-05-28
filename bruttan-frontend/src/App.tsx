import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from './assets/vite.svg'
import heroImg from './assets/hero.png'
import './App.css'
import createFetchClient from "openapi-fetch";
import createClient from "openapi-react-query";
import { useGet, useGetHej } from './api/default/default'
import type { paths } from './api/v1'





const fetchClient = createFetchClient<paths>({
  baseUrl: "http://localhost:3000/api",
});
const $api = createClient(fetchClient);

function App() {
  const [count, setCount] = useState(0)

  const { data, error, isLoading } = $api.useQuery("get", "/hesdflkjj");



  // const { data: num, isLoading: numLoading, error: numErr } = useGetHej();
  // const { data, isLoading, error } = useGet();

  if (isLoading) return (<p> Loading...</p>)


  console.log(data)

  if (error) {
    console.log(error.c)
    return <p>Error fuk</p>
  }


  return (
    <>
      <h1> Hejsan svejsan </h1>

      <p>{data?.hej}</p>
      <p>what defuk</p>


    </>
  )
}

export default App
