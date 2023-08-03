import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { emit } from '@tauri-apps/api/event'


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
    <div className="flex mt-20 h-screen justify-start space-x-1 mx-auto w-full">
      <div
        className='flex h-12 font-bold text-xl items-center justify-center w-20 mr-8 ml-4'>
        Duration
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
  )
}

export default Settings
