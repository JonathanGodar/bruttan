import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from './assets/vite.svg'
import heroImg from './assets/hero.png'
import './App.css'
import { useGet, useGetHej } from './api/default/default'

function App() {
  const [count, setCount] = useState(0)

  const { data: num, isLoading: numLoading, error: numErr } = useGetHej();
  // const { data, isLoading, error } = useGet();

  if (numLoading) return (<p> Loading...</p>)


  console.log(num)

  if (numErr) {
    console.log(numErr)
    return <p>Error fuk</p>
  }


  return (
    <>
      <h1> Hejsan svejsan </h1>

      <p>{num?.hej}</p>
      <p>what defuk</p>




    </>
  )
}

export default App
