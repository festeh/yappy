"use client"

import React from 'react'
import {useEffect, useState} from 'react'

const TIMES = [5, 10, 15, 20, 25, 30].map(t => t * 60);

const Settings = () => {

  function onDurationChange(e) {
    setDuration(e.target.value);
  }

  const [duration, setDuration] = useState(-1);

  useEffect(() => {
    
  }, [])

  return (
    <div className="flex mt-20 h-screen justify-start space-x-1 mx-auto w-full">
      <div 
    className='flex h-12 font-bold text-xl items-center justify-center w-20 mr-8 ml-4'>
        Duration
      </div>
      {
        TIMES.map(t => (
          <div className="flex rounded-md shadow-xl items-center px-2 h-12 w-20 justify-center bg-green-300">
            <span className="font-bold text-center">{t / 60}</span>
          </div>
        ))
      }
    </div>
  )
}

export default Settings
