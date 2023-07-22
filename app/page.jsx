"use client"

import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import Link from "next/link";

import {
  FaPlayCircle,
  FaPauseCircle,
  FaMinusCircle,
} from "react-icons/fa";

const Pomo = () => {
  function handleClick() {
    if (!running) {
      invoke('run')
    } else {
      invoke('pause')
    }
  }

  function handleReset() {
    invoke('reset')
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
    listen('pomo_reseted', (e) => {
      setRunning(false);
      setTimer(invoke('get_duration'));
    });
    listen('pomo_step', (e) => {
      const pomoTime = e.payload;
      setTimer(pomoTime);
    });
  }, [])

  return (
    <div className='flex flex-col w-full align-center mt-16 h-screen'>
      <div className='text-8xl mx-auto w-full text-black  text-center'>{timer}</div>
      <div className='flex justify-center align-center mx-auto'>
        {running ?
          <FaPauseCircle size={160} color="green" className='mt-4 w-48 rounded' onClick={handleClick} />
          :
          <FaPlayCircle size={160} color="green" className='mt-4  w-48 rounded' onClick={handleClick} />
        }
        <FaMinusCircle size={160} color="red" className='mt-4 w-48 rounded' onClick={handleReset} />
      </div>
    </div>
  )
}

export default Pomo
