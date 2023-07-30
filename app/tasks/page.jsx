"use client"

import { IoReloadCircle } from 'react-icons/io5';
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { useEffect, useState } from 'react'

const Tasks = () => {

  function onClick() {
    invoke('load_tasks')
  }

  const [tasks, setTasks] = useState([])

  useEffect(() => {
    listen('tasks_loaded', (e) => {
      console.log(e)
      setTasks(e.payload)
    })
  }, [])

  return (
    <div className="border">
      <div className="flex items-end flex-end justify-end">
        <IoReloadCircle size={100} color="blue" className='mr-4 rounded' onClick={onClick} />
      </div>

      <div>
        <ul id="tasks">
          {tasks.map(task => (

            <li key={task.id}>{task.content}</li>))}
        </ul>
      </div>

    </div>
  )
}

export default Tasks
