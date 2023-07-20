"use client"


import { useEffect, useState } from 'react'
import { listen, emit } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'

import {
  FaPlayCircle,
  FaPauseCircle,
} from "react-icons/fa";
//
function secondsToPomoTime(payload) {
  const minutes = Math.floor(payload / 60) % 60
  let seconds = payload % 60
  return `${minutes.toString().padStart(2, "0")}:${seconds.toString().padStart(2, "0")}`
}
//
const Pomo = () => {
  function handleClick() {
    if (!running) {
      invoke('run')
    } else {
      invoke('pause')
    }
  }

  const [timer, setTimer] = useState("05:00");
  const [running, setRunning] = useState(false);

  useEffect(() => {
    console.log("Boom")
    listen('pomo_started', (e) => {
      setRunning(true);
    }, [])
    listen('pomo_finished', (e) => {
      setRunning(false);
    }, [])
    listen('pomo_paused', (e) => {
      setRunning(false);
    }, [])
    listen('pomo_step', (e) => {
      console.log(e)
      const pomoTime = secondsToPomoTime(e.payload);
      setTimer(pomoTime);
    }, [])
  })

  return (
    <div className='flex flex-col w-full align-center justify-center h-screen bg-white border p-4'>
      <div className='text-8xl mx-auto w-full text-black  text-center'>{timer}</div>
      {running ?
        <FaPauseCircle size={160} color="green" className='mt-4 mx-auto w-48 rounded' onClick={handleClick} />
        :
        <FaPlayCircle size={160} color="green" className='mt-4 mx-auto w-48 rounded' onClick={handleClick} />
      }

    </div>
  )
}

export default Pomo
