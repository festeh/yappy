import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { emit } from '@tauri-apps/api/event'
import { Divider } from 'antd';

import ApiKeyForm from './components/ApiKeyForm'

const TIMES = [5, 10, 15, 20, 25, 30].map(t => t * 60);

const Settings = () => {

  function onDurationChange(e) {
    let d = +e.target.innerText
    emit('duration_changed', d * 60)
    setDuration(d * 60)
  }

  const [duration, setDuration] = useState(0);

  useEffect(() => {
    invoke('get_duration_seconds').then(d => setDuration(d));
  }, [])

  return (
    <div className="h-full">
      <Divider />
      <div className="flex mt-8 mb-8 justify-start space-x-1 mx-auto w-full">
        <div
          className='flex h-12 font-bold text-xl  justify-left w-48 mr-8 ml-4'>
          Pomodoro Duration
        </div>
        {
          TIMES.map(t => {
            let color = "bg-green-600";
            if (t === duration) {
              color = "bg-red-600";
            }
            return <div key={t} className={`${color} flex rounded-md shadow-xl items-center px-2 h-12 w-20 justify-center font-bold`} onClick={onDurationChange}>
              {t / 60}
            </div>
          })
        }
      </div>
      <Divider />
      <div className="mt-8 flex">
        <div
          className='flex h-12 font-bold text-xl justify-left w-44 mr-12 ml-4'>
          Todoist API Key
        </div>
        <ApiKeyForm />
      </div>
    </div>
  )
}

export default Settings
