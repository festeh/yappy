import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'

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

  const [selectedTask, setSelectedTask] = useState("Select a task")
  const [timer, setTimer] = useState("");
  const [running, setRunning] = useState(false);

  const Initialize = async () => {

    const dur = await invoke('get_duration');
    setTimer(dur);

    invoke('get_selected_task').then((task) => { setSelectedTask(task) }).catch((err) => {

    })

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
      invoke('get_duration').then((dur) => {
        setTimer(dur);
      })
    });
    listen('pomo_step', (e) => {
      const pomoTime = e.payload;
      setTimer(pomoTime);
    });

  }

  useEffect(() => {
    Initialize();
  }, [])


  return (
    <div className='flex mt-40 flex-col w-full align-center h-screen'>
      <div className='text-2xl text-center mb-8'>
        {selectedTask}
      </div>
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
