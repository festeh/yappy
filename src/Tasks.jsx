import { IoReloadCircle } from 'react-icons/io5';
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { useEffect, useState } from 'react'
import { List, Typography } from 'antd'

const Tasks = () => {

  function onReloadClick() {
    invoke('reload_tasks')
  }

  function onSelectClick(e) {
    console.log(e.target.id)
    invoke('select_task', { id: e.target.id })
  }

  const [tasks, setTasks] = useState([])

  useEffect(() => {
    invoke('get_tasks').then((tasks) => {
      console.log("New tasks", tasks)
      setTasks(tasks)
    })
    listen('tasks_loaded', (e) => {
      setTasks(e.payload)
    })
  }, [])

  return (
    <div className="border">
      <div className="flex items-end flex-end justify-end">
        <IoReloadCircle size={90} color="blue" className='mr-4 rounded' onClick={onReloadClick} />
      </div>
      <div className="mt-2">
        <List dataSource={tasks}
          size="large"
          bordered
          renderItem={(task) => (
            <List.Item id={task.id} onClick={onSelectClick} className="text-xl hover:bg-fuchsia-600" key={task.id}>
              {task.content}
            </List.Item>
          )}>
        </List>
      </div>
    </div>
  )
}

export default Tasks
