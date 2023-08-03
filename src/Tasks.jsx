
import { IoReloadCircle } from 'react-icons/io5';
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { useEffect, useState } from 'react'
import { List, Typography } from 'antd'

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
        <IoReloadCircle size={90} color="blue" className='mr-4 rounded' onClick={onClick} />
      </div>
      <div className="mt-2">
        <List dataSource={tasks}
          size="large"
          bordered
          renderItem={(task) => (
            <List.Item className="text-xl" key={task.id}>
              {task.content}
            </List.Item>
          )}>
        </List>
      </div>
    </div>
  )
}

export default Tasks
