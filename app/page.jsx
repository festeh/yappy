"use client"

import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'

import {
  FaPlayCircle,
  FaPauseCircle,
} from "react-icons/fa";

const Pomo = () => {
  function handleClick() {
    if (!running) {
      invoke('run')
    } else {
      invoke('pause')
    }
  }

  const [timer, setTimer] = useState("");
  const [running, setRunning] = useState(false);

  useEffect(() => {
    const dur = invoke('get_duration');
    setTimer(dur);
    listen('pomo_started', (e) => {
      setRunning(true);
    });
    listen('pomo_finished', (e) => {
      setRunning(false);
    });
    listen('pomo_paused', (e) => {
      setRunning(false);
    });
    listen('pomo_step', (e) => {
      const pomoTime = e.payload;
      setTimer(pomoTime);
    });
  }, [])

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
